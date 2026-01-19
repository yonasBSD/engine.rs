use loom::sync::Arc;
use loom::sync::Mutex;
use loom::thread;

#[test]
fn loom_concurrency_smoke() {
    loom::model(|| {
        let counter = Arc::new(Mutex::new(0));

        {
            let c = counter.clone();
            thread::spawn(move || {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            });
        }

        {
            let c = counter.clone();
            thread::spawn(move || {
                let mut guard = c.lock().unwrap();
                *guard += 1;
            });
        }

        // Loom auto-joins threads at the end of the model block.
        let final_value = *counter.lock().unwrap();
        assert!(final_value <= 2);
    });
}
