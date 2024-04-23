extern crate indicatif;

use self::indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use std::collections::HashMap;

pub struct Progress {
    pub multi_progress: MultiProgress,
    pub progress_bars: HashMap<usize, ProgressBar>,
}

impl Default for Progress {
    fn default() -> Progress {
        Progress {
            multi_progress: MultiProgress::new(),
            progress_bars: HashMap::new(),
        }
    }
}

impl Progress {
    pub fn add(&mut self, range: usize, thread_number: usize) {
        let pb = self.multi_progress.add(ProgressBar::new(range as u64));
        let style: ProgressStyle = ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta} remaining)  {msg}").unwrap()
            .progress_chars("##-");
        pb.set_style(style);
        pb.set_message(format!("thread #{}", thread_number));
        self.progress_bars.insert(thread_number, pb);
    }

    pub fn dump(&self) -> HashMap<&usize, (u64, bool)> {
        let mut result = HashMap::new();
        for (k, v) in self.progress_bars.iter() {
            result.insert(k, (v.position(), v.is_finished()));
        }
        result
    }

    pub fn inc(&self, amount: usize, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.inc(amount as u64);
    }

    pub fn set_position(&self, amount: u64, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.set_position(amount);
    }

    pub fn is_finished(&self, thread_number: usize) -> bool {
        match self.progress_bars.get(&thread_number) {
            Some(x) => x.is_finished(),
            None => false,
        }
    }

    pub fn finish(&self, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.finish_with_message("--done--");
    }

    pub fn join_and_clear(&self) {
        let _ = self.multi_progress.clear();
    }
}
