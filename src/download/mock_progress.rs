extern crate indicatif;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Progress {
    multi_progress: Vec<Arc<RwLock<ProgressBar>>>,
    progress_bars: HashMap<usize, Arc<RwLock<ProgressBar>>>,
    inner_progress: Option<Arc<RwLock<self::indicatif::ProgressBar>>>,
}

#[warn(dead_code)]
struct ProgressBar {
    position: u64,
    is_finished: bool,
    total_len: u64,
}

impl Default for Progress {
    fn default() -> Progress {
        Progress {
            multi_progress: Vec::new(),
            progress_bars: HashMap::new(),
            inner_progress: None,
        }
    }
}

impl Progress {
    fn reset_inner(&self) {
        if let Some(pb) = self.inner_progress.as_ref() {
            let mut total = 0;
            let mut total_len = 0;
            let mut all_finished = true;
            for sub in self.multi_progress.iter() {
                let pbr = sub.read().unwrap();
                total += pbr.position;
                all_finished &= pbr.is_finished;
                total_len += pbr.total_len;
            }
            let pbw = pb.write().unwrap();
            pbw.set_position(total);
            pbw.set_length(total_len);
            if all_finished {
                pbw.finish();
            }
        }
    }

    pub fn with_bar(bar: Option<self::indicatif::ProgressBar>) -> Progress {
        Progress {
            multi_progress: Vec::new(),
            progress_bars: HashMap::new(),
            inner_progress: match bar {
                Some(inner) => Some(Arc::new(RwLock::new(inner))),
                None => None,
            },
        }
    }
    pub fn add(&mut self, range: usize, thread_number: usize) {
        let progress_bar = Arc::new(RwLock::new(ProgressBar {
            position: 0,
            is_finished: false,
            total_len: range as u64,
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
        // if let Some(pb) = self.inner_progress.as_ref() {
        //     pb.write().unwrap().inc(amount as u64);
        // }
        self.reset_inner();
    }

    pub fn set_position(&mut self, amount: u64, thread_number: usize) {
        let pb = match self.progress_bars.get(&thread_number) {
            Some(x) => x,
            None => return,
        };
        pb.write().unwrap().position = amount;
        self.reset_inner();
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
        pb.write().unwrap().is_finished = true;
    }
    // pub fn check_finish(&mut self) {
    //     self.reset_inner();
    // }
}
