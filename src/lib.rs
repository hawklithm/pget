use std::path::PathBuf;

pub mod common;
mod download;

/// download file paraA file download interface that supports resumable downloads and concurrency.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// let p = PathBuf::from("test");
/// let p = p.join("merges_doc.txt");
/// pget::download(
/// "https://hf-mirror.com/NexaAIDev/Octopus-v2/resolve/main/tokenizer.model".to_string(),
/// num_cpus::get(),
/// p.clone(),
/// ).unwrap();
/// ```
pub fn download<P: AsRef<str>>(
    url: P,
    thread: usize,
    output_file: PathBuf,
) -> common::error::Result<()> {
    let download = download::Download {
        threads: thread,
        url: url.as_ref().to_owned(),
        filename: output_file,
        keep_cache: true,
        ..Default::default()
    };

    download.get()
}

/// download file paraA file download interface that supports resumable downloads and concurrency.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// let p = PathBuf::from("test");
/// let p = p.join("merges.txt");
/// pget::download_with_cleaner(
/// "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/merges.txt".to_string(),
/// num_cpus::get(),
/// p.clone(),
/// true
/// ).unwrap();
/// ```
pub fn download_with_cleaner<P: AsRef<str>>(
    url: P,
    thread: usize,
    output_file: PathBuf,
    keep_cache: bool,
) -> common::error::Result<()> {
    let download = download::Download {
        threads: thread,
        url: url.as_ref().to_owned(),
        filename: output_file,
        keep_cache,
        ..Default::default()
    };

    download.get()
}

#[cfg(not(feature = "progress_bar"))]
use indicatif::ProgressBar;

/// download file with custom progress bar and supports resumable downloads and concurrency.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use indicatif::ProgressBar;
/// use indicatif::ProgressStyle;
/// use sha2::Digest;
/// use sha2::Sha256;
/// let progress = ProgressBar::new(0);
/// progress.set_style(
/// ProgressStyle::with_template(
/// "{msg} [{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})",
///             )
///                 .unwrap(),
///         );
/// progress.set_message("tokenizer");
/// let url =
/// "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/tokenizer.json".to_string();
/// let p = PathBuf::from("test");
/// let p = p.join("qwen.safetensors");
/// pget::download_with_custom_progress(url, 4, p.clone(), Some(progress),true).unwrap();
/// ```
///
#[cfg(not(feature = "progress_bar"))]
pub fn download_with_custom_progress<P: AsRef<str>>(
    url: P,
    thread: usize,
    output_file: PathBuf,
    progress: Option<ProgressBar>,
    keep_cache: bool,
) -> common::error::Result<()> {
    let download = download::Download {
        threads: thread,
        url: url.as_ref().to_owned(),
        filename: output_file,
        progress: crate::download::mock_progress::Progress::with_bar(progress),
        keep_cache,
        ..Default::default()
    };

    download.get()
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read, path::PathBuf};

    use indicatif::ProgressBar;
    use indicatif::ProgressStyle;
    use sha2::Digest;
    use sha2::Sha256;

    use crate::download;
    use crate::download_with_custom_progress;

    fn calc_sha256(file: PathBuf) -> crate::common::error::Result<String> {
        // 创建一个新的Sha256对象
        let mut hasher = Sha256::new();

        // 打开文件进行读取
        let mut file = File::open(file)?;

        // 一次性读取文件所有内容并更新哈希状态
        let mut buffer = [0; 1024];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        // 完成哈希计算
        let result = hasher.finalize();

        // 将哈希结果转换为十六进制字符串
        let hash_hex = hex::encode(result);
        return Ok(hash_hex);
    }

    #[test]
    fn test() {
        let p = PathBuf::from("test");
        let p = p.join("tokenizer.model");
        download(
            "https://hf-mirror.com/Qwen/CodeQwen1.5-7B/resolve/main/tokenizer.model?download=true"
                .to_string(),
            8,
            p.clone(),
        )
        .unwrap();

        assert_eq!(
            calc_sha256(p).unwrap(),
            "656b66a920a54bc45e8e06dc587691ab3c0b2930b9ae56d5fa31e72db2f3bff3"
        );
    }

    #[test]
    fn test2() {
        let p = PathBuf::from("test");
        let p = p.join("merges.txt");
        download(
            "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/merges.txt".to_string(),
            num_cpus::get(),
            p.clone(),
        )
        .unwrap();

        assert_eq!(
            calc_sha256(p).unwrap(),
            "599bab54075088774b1733fde865d5bd747cbcc7a547c5bc12610e874e26f5e3"
        );
    }

    #[test]
    fn test3() {
        let p = PathBuf::from("test");
        let p = p.join("Octopus-v2.bin");
        download(
            "https://hf-mirror.com/NexaAIDev/Octopus-v2/resolve/main/tokenizer.model?download=true"
                .to_string(),
            4,
            p.clone(),
        )
        .unwrap();
        assert_eq!(
            calc_sha256(p).unwrap(),
            "61a7b147390c64585d6c3543dd6fc636906c9af3865a5548f27f31aee1d4c8e2"
        );
    }

    #[test]
    #[cfg(not(feature = "progress_bar"))]
    fn test4() {
        let progress = ProgressBar::new(0);
        progress.set_style(
                ProgressStyle::with_template(
                    "{msg} [{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})",
                )
                    .unwrap(),
            );
        progress.set_message("tokenizer");
        let url =
            "https://hf-mirror.com/ai21labs/Jamba-v0.1/resolve/main/tokenizer.model?download=true"
                .to_string();
        let p = PathBuf::from("test");
        let p = p.join("Jamba.bin");
        download_with_custom_progress(url, 4, p.clone(), Some(progress), true).unwrap();
        assert_eq!(
            calc_sha256(p).unwrap(),
            "02fd6530b8ede0eedd8e509fcab32da7b1dd04c8119f8498c787100f13112713"
        );
    }

    #[test]
    fn test_download_with_cleaner() {
        use crate::download_with_cleaner;
        let url =
            "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/tokenizer.json".to_string();
        let p = PathBuf::from("test");
        let p = p.join("qwen.safetensors");
        download_with_cleaner(url, 4, p.clone(), false).unwrap();
        assert_eq!(
            calc_sha256(p).unwrap(),
            "f7c9b2dba4a296b1aa76c16a34b8225c0c118978400d4bb66bff0902d702f5b8"
        );
    }
}
