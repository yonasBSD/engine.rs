#[cfg(feature = "loom")]
pub use loom::sync::{
    Arc,
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[cfg(not(feature = "loom"))]
pub use std::sync::{
    Arc,
    Mutex,
    atomic::{AtomicUsize, Ordering},
};
