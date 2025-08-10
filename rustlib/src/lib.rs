mod job_manager;  // job_manager.rsをモジュールとして読み込む

use job_manager::JobManager;
use std::sync::OnceLock;

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
}
