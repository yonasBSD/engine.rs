use std::{
    io::{self, Write},
    path::Path,
    thread,
    time::Duration,
};

use console::style;
use engine_rs_lib::traits::FileSystem;

// ==========================================================
// 1. UI DECORATOR (LoggingFS)
// ==========================================================
/// A decorator for any `FileSystem` that updates the Clack progress bar
pub struct LoggingFS<'a, F: FileSystem> {
    inner: F,
    pb: &'a cliclack::ProgressBar,
}

impl<'a, F: FileSystem> LoggingFS<'a, F> {
    pub const fn new(inner: F, pb: &'a cliclack::ProgressBar) -> Self {
        Self {
            inner,
            pb,
        }
    }

    pub fn log_ephemeral(&self, action: &str, path: &Path) {
        self.pb.inc(1);
        // ANSI escape codes to clear the line and print the update above the bar
        print!(
            "\x1B[1A\x1B[2K\r  {} {}: {:?}\n",
            style("⚡").yellow(),
            action,
            path
        );
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(5));
    }

    pub fn clear_ui_lines(&self) {
        print!("\x1B[1A\x1B[2K\r\x1B[1A\x1B[2K\r");
        let _ = io::stdout().flush();
    }
}

impl<F: FileSystem> FileSystem for LoggingFS<'_, F> {
    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        self.log_ephemeral("mkdir", path);
        self.inner.create_dir_all(path)
    }
    fn write_file(&self, path: &Path, content: &str) -> io::Result<()> {
        self.log_ephemeral("write", path);
        self.inner.write_file(path, content)
    }
    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.inner.read_to_string(path)
    }
}
