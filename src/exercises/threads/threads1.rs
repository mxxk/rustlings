// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: std::sync::atomic::AtomicU32,
}

pub fn main() {
    let mem_ord_relx = std::sync::atomic::Ordering::Relaxed;
    let status = Arc::new(JobStatus { jobs_completed: 0.into() });
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            status_shared.jobs_completed.fetch_add(1, mem_ord_relx);
        }
    });
    while status.jobs_completed.load(mem_ord_relx) < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
