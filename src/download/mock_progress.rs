extern crate indicatif;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Progress {
    multi_progress: Vec<Arc<RwLock<ProgressBar>>>,
    progress_bars: HashMap<usize, Arc<RwLock<ProgressBar>>>,
}

#[warn(dead_code)]
struct ProgressBar {
    position: u64,
    is_finished: bool,
}

impl Default for Progress {
    fn default() -> Progress {
        Progress {
            multi_progress: Vec::new(),
            progress_bars: HashMap::new(),
        }
    }
}

impl Progress {
    pub fn add(&mut self, _range: usize, thread_number: usize) {
        let progress_bar = Arc::new(RwLock::new(ProgressBar {
            position: 0,
            is_finished: false,
        }));
        self.multi_progress.push(progress_bar.clone());
        self.progress_bars.insert(thread_number, progress_bar);
    }

    pub fn dump(&self) -> HashMap<&usize, (u64, bool)> {
        let mut result = HashMap::new();
        for (k, v) in self.progress_bars.iter() {
            result.insert(
                k,
                (v.read().unwrap().position, v.read().unwrap().is_finished),
            );
        }
        result
    }

    pub fn inc(&self, amount: usize, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.write().unwrap().position += amount as u64;
    }

    pub fn set_position(&mut self, amount: u64, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.write().unwrap().position = amount;
    }

    pub fn is_finished(&self, thread_number: usize) -> bool {
        match self.progress_bars.get(&thread_number) {
            Some(x) => x.read().unwrap().is_finished,
            None => false,
        }
    }

    pub fn finish(&self, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.write().unwrap().is_finished = true
    }
}
