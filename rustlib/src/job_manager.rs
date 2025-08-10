use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Job {
    progress: u32,
    canceled: bool,
}

pub struct JobManager {
    jobs: Mutex<HashMap<u64, Arc<Mutex<Job>>>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self { jobs: Mutex::new(HashMap::new()) }
    }

    pub fn start_job(&self, job_id: u64) {
        let job = Arc::new(Mutex::new(Job { progress: 0, canceled: false }));
        self.jobs.lock().unwrap().insert(job_id, job.clone());

        thread::spawn(move || {
            for i in 0..=100 {
                {
                    let job = job.lock().unwrap();
                    if job.canceled {
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(100)); // 重い処理の代わり
                let mut job = job.lock().unwrap();
                job.progress = i;
            }
        });
    }

    pub fn cancel_job(&self, job_id: u64) {
        if let Some(job) = self.jobs.lock().unwrap().get(&job_id) {
            let mut job = job.lock().unwrap();
            job.canceled = true;
        }
    }

    pub fn get_progress(&self, job_id: u64) -> u32 {
        if let Some(job) = self.jobs.lock().unwrap().get(&job_id) {
            return job.lock().unwrap().progress;
        }
        0
    }
}
