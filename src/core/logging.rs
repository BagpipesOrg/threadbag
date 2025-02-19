//use tracing_subscriber;
//use tracing_appender::non_blocking::WorkerGuard;

use tracing::{debug, info, Level};
use tracing_appender::rolling;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[derive(Debug, Clone)]
pub struct Logger;

impl Logger {
    /// Initialize the logger with a file appender and console output
    pub fn init() {
        // Create a file appender that rotates logs daily in the "logs" directory
        let file_appender = rolling::daily("/var/log", "threadbag.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Create a subscriber that writes to both the console and the file
        let file_subscriber = fmt::Layer::new()
            .with_writer(non_blocking)
            .with_ansi(false) // Disable ANSI colors for file output
            .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

        let console_subscriber = fmt::Layer::new()
            .with_writer(std::io::stdout)
            .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

        // Combine the subscribers
        tracing_subscriber::registry()
            .with(file_subscriber)
            .with(console_subscriber)
            .init();
    }

    /// Log a debug message
    pub fn log_debug(&self, message: &str) {
        debug!("{}", message);
    }

    /// Log an info message
    pub fn log_info(&self, message: &str) {
        info!("{}", message);
    }
}
