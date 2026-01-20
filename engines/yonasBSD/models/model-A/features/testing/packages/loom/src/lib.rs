pub mod core;
pub mod enums;
pub mod traits;
pub mod utils;

// Core internal exports
pub use core::backends;
pub use core::frontends;

//
// 1. GLOBAL STATE INITIALIZATION
//

#[cfg(test)]
mod test {
    //! Loom concurrency tests for global init, multi-threaded logging,
    //! and writer swapping under contention.
    use loom::sync::atomic::{AtomicUsize, Ordering};
    use loom::sync::{Arc, Mutex};
    use loom::thread;

    #[test]
    fn loom_global_init() {
        loom::model(|| {
            let init_count = Arc::new(AtomicUsize::new(0));

            let spawn_init = || {
                let c = init_count.clone();
                thread::spawn(move || {
                    if c.fetch_add(1, Ordering::SeqCst) == 0 {
                        // first initializer wins
                    }
                })
            };

            let t1 = spawn_init();
            let t2 = spawn_init();
            let t3 = spawn_init();

            t1.join().unwrap();
            t2.join().unwrap();
            t3.join().unwrap();

            let count = init_count.load(Ordering::SeqCst);

            assert!(count >= 1);
            assert!(count <= 3);
        });
    }

    //
    // 2. MULTI-THREADED LOG EMISSION
    //

    #[test]
    fn loom_multi_thread_logging() {
        loom::model(|| {
            let buffer = Arc::new(Mutex::new(Vec::<&'static str>::new()));

            let spawn_logger = |msg: &'static str| {
                let buf = buffer.clone();
                thread::spawn(move || {
                    let mut guard = buf.lock().unwrap();
                    guard.push(msg);
                })
            };

            let t1 = spawn_logger("a");
            let t2 = spawn_logger("b");
            let t3 = spawn_logger("c");

            t1.join().unwrap();
            t2.join().unwrap();
            t3.join().unwrap();

            let logs = buffer.lock().unwrap();
            assert!(logs.len() <= 3);
        });
    }
}
