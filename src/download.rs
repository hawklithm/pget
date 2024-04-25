use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, SeekFrom};
use std::io::{prelude::*, BufWriter};
use std::path::{self, Path, PathBuf};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use futures::future::join_all;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;

use crate::common;
use crate::common::error::DownloadError;

use self::network::Network;
use self::progress::Progress;

mod network;
mod progress;

// static ONE_KB: u64 = 1024;

static CACHE_STATUS_FILE: &str = "download_status.json";

pub(crate) struct Download {
    pub url: String,
    pub filename: PathBuf,
    // pub memory: u64,
    pub threads: usize,
    pub network: network::Network,
    pub progress: progress::Progress,
}

fn copy_n_byte<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W, len: usize) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let bf_len = 1024;
    let mut count = 0;
    if len > bf_len {
        let mut buffer = vec![0u8; 1024];
        while count + bf_len <= len {
            reader.read_exact(&mut buffer)?;
            count += bf_len;
            writer.write(&buffer)?;
        }
    }
    let rest = len % bf_len;
    let mut rest_buf = vec![0u8; rest];
    reader.read_exact(&mut rest_buf)?;
    writer.write(&rest_buf)?;
    writer.flush()?;
    count += rest;
    Ok(count as u64)
}

#[derive(Deserialize, Serialize)]
struct DownloadProcess {
    pub thread: usize,
    pub cached_size: u64,
    pub finished: bool,
}

impl Default for Download {
    fn default() -> Download {
        Download {
            url: "".to_string(),
            filename: PathBuf::from("".to_string()),
            // memory: 256,
            threads: 4,
            network: network::Network::default(),
            progress: progress::Progress::default(),
        }
    }
}

fn load_json<P: AsRef<Path>>(file_path: P) -> Option<Vec<DownloadProcess>> {
    let file_content = std::fs::read_to_string(file_path).ok()?;
    let processes: Vec<DownloadProcess> = serde_json::from_str(&file_content).ok()?;
    return Some(processes);
}
fn hash_string_to_hex(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash_result = hasher.finish();
    format!("{:x}", hash_result)
}

impl Download {
    pub fn get(self) -> common::error::Result<()> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(self.threads)
            .thread_name("pget")
            .enable_all()
            .build()?;
        let content_length_resp: Option<u64> =
            rt.block_on(self.network.get_content_length(&self.url))?;

        match content_length_resp {
            Some(content_length) => {
                let target_filename = self.filename.clone();
                let children = Download::spawn_threads(self, &rt,content_length as usize)?;
                let request_result = rt.block_on(join_all(children)).into_iter().filter_map(|x|x.ok()).filter_map(|x|x.ok()).collect::<Vec<_>>();
                Download::assemble(target_filename.clone(), request_result)?;
                let target_file_handle = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(target_filename)?;
                target_file_handle.set_len(content_length)?;
            }
            None => println!("Content length is not present for this URL. Support for this type of hosted file will be added in the future."),
        }
        Ok(())
    }

    fn assemble(
        file_path: PathBuf,
        ranges: Vec<(String, usize, usize)>,
    ) -> common::error::Result<()> {
        let origin_file_path_ref = file_path.clone();
        let origin_file_handle = OpenOptions::new()
            .write(true)
            .create(true)
            .open(origin_file_path_ref)?;
        let origin_file_arc = Arc::new(origin_file_handle);
        for (cache_file_name, range_start, range_end) in ranges {
            let mut origin_file_ref = origin_file_arc.clone();
            origin_file_ref.seek(SeekFrom::Start(range_start as u64))?;
            let mut cache_file_handle = File::open(&cache_file_name)?;
            let mut writer = BufWriter::new(origin_file_ref);
            copy_n_byte(&mut cache_file_handle, &mut writer, range_end - range_start)?;
            writer.flush()?;
        }
        Ok(())
    }

    fn calculate_ranges(
        threads: usize,
        content_length: usize,
        mut progress: progress::Progress,
        cache_dir: PathBuf,
    ) -> (
        progress::Progress,
        Vec<(Option<String>, usize, usize, usize)>,
        HashMap<usize, DownloadProcess>,
    ) {
        let mut range_start = 0;
        let mut ranges = vec![];
        let chunk_size = content_length / threads - 1;

        let record_status = Self::load_process(threads, cache_dir.clone());
        let mut map = HashMap::new();
        for record in record_status {
            map.insert(record.thread, record);
        }

        for thread in 0..threads {
            let mut range_end = chunk_size + range_start;
            if thread == (threads - 1) {
                range_end = content_length
            }

            let range_to_process = range_end - range_start;

            let thread_number = thread + 1;
            let range_start_option = if let Some(process) = map.get(&thread_number) {
                if process.cached_size as usize >= range_end - range_start {
                    None
                } else {
                    Some(range_start + process.cached_size as usize)
                }
            } else {
                Some(range_start)
            };
            progress.add(range_to_process, thread_number);
            if let Some(range_start_to_query) = range_start_option {
                let range: String = format!("bytes={}-{}", range_start_to_query, range_end - 1);
                // println!("   Thread: {}, range: {}, chunks: {}, chunk_remainder: {}", thread_number, range, buffer_chunks, chunk_remainder);
                ranges.push((Some(range), range_start, thread_number, range_end));
            } else {
                ranges.push((None, range_start, thread_number, range_end));
                progress.finish(thread_number);
            }
            if let Some(process) = map.get(&thread_number) {
                if process.finished || progress.is_finished(thread_number) {
                    progress.finish(thread_number);
                } else {
                    progress.set_position(process.cached_size, thread_number);
                }
            };

            range_start = range_start + chunk_size;
        }
        return (progress, ranges, map);
    }

    async fn request(
        file_handle: &mut File,
        progress_ref: Arc<Progress>,
        network_ref: Arc<Network>,
        thread_number: usize,
        url_ref: String,
        range: String,
    ) -> common::error::Result<()> {
        let mut file_range_resp = network_ref.make_request(&url_ref, Some(range)).await?;
        while let Some(chunk) = file_range_resp.chunk().await? {
            let buffer_size = chunk.len();
            file_handle.write(&chunk)?;
            file_handle.flush()?;
            progress_ref.inc(buffer_size, thread_number);
        }

        Ok(())
    }

    fn dump_process(
        current: HashMap<&usize, (u64, bool)>,
        cached_dir: PathBuf,
    ) -> common::error::Result<()> {
        let mut process_status = Vec::new();
        for (key, (value, finished)) in current {
            process_status.push(DownloadProcess {
                thread: *key,
                cached_size: value,
                finished,
            });
        }
        let json_str = serde_json::to_string_pretty(&process_status)?;
        let status_file = cached_dir.join(CACHE_STATUS_FILE);
        fs::write(status_file, json_str)?;
        Ok(())
    }

    fn load_process(threads: usize, cached_dir: PathBuf) -> Vec<DownloadProcess> {
        let status_file = cached_dir.join(CACHE_STATUS_FILE);
        if let Some(status) = load_json(status_file) {
            if status.len() == threads {
                return status;
            }
        }
        let mut initial_status = Vec::new();
        for idx in 0..threads {
            initial_status.push(DownloadProcess {
                thread: idx,
                cached_size: 0,
                finished: false,
            })
        }
        initial_status
    }

    fn spawn_threads(
        self,
        rt: &Runtime,
        content_length: usize,
    ) -> common::error::Result<Vec<JoinHandle<common::error::Result<(String, usize, usize)>>>> {
        let mut children = vec![];

        let network_arc = Arc::new(self.network);
        let file_path = self.filename;
        let file_dir = file_path.parent().ok_or(DownloadError::parameter(
            "target file should have a parent dir",
        ))?;
        let hash_name = hash_string_to_hex(&self.url);
        let cache_dir = file_dir.join(".cache").join(hash_name);
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        let (progress, ranges, map) = Download::calculate_ranges(
            self.threads,
            content_length,
            self.progress,
            cache_dir.clone(),
        );
        let progress_arc = Arc::new(progress);

        // check file
        let _target = File::create(file_path.clone())?;

        let file_name = file_path
            .file_name()
            .ok_or(DownloadError::parameter(
                "target file should not be a director",
            ))?
            .to_str()
            .unwrap()
            .to_string();

        let map_arc = Arc::new(map);

        for (range_opt, range_start, thread_number, range_end) in ranges.clone() {
            let progress_ref = progress_arc.clone();
            let network_ref = network_arc.clone();
            let url_ref = self.url.clone();
            let file_name_ref = file_name.clone();
            let cache_path_ref = cache_dir.to_str().unwrap().to_string();
            let map_ref = map_arc.clone();

            children.push(rt.spawn(async move {
                let cache_file_name = format!(
                    "{}{}{}.{}",
                    cache_path_ref,
                    path::MAIN_SEPARATOR,
                    file_name_ref,
                    thread_number
                );
                if let Some(range) = range_opt {
                    if !progress_ref.is_finished(thread_number) {
                        let mut cache_file_handle = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(&cache_file_name)?;
                        let _ = cache_file_handle.set_len((range_end - range_start) as u64);
                        if let Some(process) = map_ref.get(&thread_number) {
                            cache_file_handle.seek(SeekFrom::Start(process.cached_size as u64))?;
                        }
                        Self::request(
                            &mut cache_file_handle,
                            progress_ref.clone(),
                            network_ref,
                            thread_number,
                            url_ref,
                            range,
                        )
                        .await?;
                        progress_ref.finish(thread_number);
                    }
                }
                return Ok((cache_file_name, range_start, range_end));
            }));
        }

        let status_checker = progress_arc.clone();
        let cache_dir_ref = cache_dir.clone();
        rt.spawn(async move {
            loop {
                thread::sleep(Duration::from_secs(1));
                let current = status_checker.dump();
                let mut all_finished = true;
                for (_, (_, is_finished)) in &current {
                    all_finished &= is_finished;
                }
                if all_finished {
                    break;
                }
                match Self::dump_process(current, cache_dir_ref.clone()) {
                    Ok(_) => (),
                    Err(e) => println!("dump process failed! error message = {:?}", e),
                }
            }
        });

        // progress_arc.clone().join_and_clear();
        return Ok(children);
    }
}
