mod job_manager;  // job_manager.rsをモジュールとして読み込む

use job_manager::JobManager;
use std::sync::OnceLock;
use std::time::Duration;
use std::thread;

static JOB_MANAGER: OnceLock<JobManager> = OnceLock::new();

#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[unsafe(no_mangle)]
pub extern "C" fn rustlib_start_job(job_id: u64) {
    let manager = JOB_MANAGER.get_or_init(JobManager::new);
    manager.start_job(job_id);
}

#[unsafe(no_mangle)]
pub extern "C" fn rustlib_cancel_job(job_id: u64) {
    if let Some(manager) = JOB_MANAGER.get() {
        manager.cancel_job(job_id);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rustlib_get_progress(job_id: u64) -> u32 {
    if let Some(manager) = JOB_MANAGER.get() {
        return manager.get_progress(job_id);
    }
    0
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn start_jobs() {
        let job_id = 999;
        rustlib_start_job(job_id);

        let mut progress = 0;
        let timeout = Duration::from_secs(60);
        let poll_interval = Duration::from_millis(200);
        let start_time = std::time::Instant::now();

        while progress < 100 {
            thread::sleep(poll_interval);
            progress = rustlib_get_progress(job_id);
            println!("Job progress: {}%", progress);

            assert!(
                progress <= 100,
                "Job progress should be between 0 and 100, got {}",
                progress
            );

            if start_time.elapsed() > timeout {
                panic!("Timeout waiting for job to complete");
            }
        }

        // ジョブ完了時のチェック
        assert_eq!(progress, 100, "Job should complete with 100% progress");

    }

}
