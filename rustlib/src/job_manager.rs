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

    /*
    pub fn start_job_with_command(&self, job_id: u64, command_json: &str) {
        let parsed: Value = serde_json::from_str(command_json).unwrap();
        let command = parsed["command"].as_str().unwrap_or("");
        let params = &parsed["params"];

        let job = Arc::new(Mutex::new(Job { progress: 0, canceled: false }));
        self.jobs.lock().unwrap().insert(job_id, job.clone());

        std::thread::spawn(move || {
            match command {
                "heavy_calc" => {
                    let x = params["x"].as_i64().unwrap_or(0);
                    let y = params["y"].as_i64().unwrap_or(0);
                    // heavy calculation logic here
                }
                "another_command" => {
                    // 別処理
                }
                _ => {
                    // 不明コマンド
                }
            }
            // 進捗や結果の管理は既存のJobManagerで対応
        });
    } */

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
