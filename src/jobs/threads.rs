use serde::Serialize;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

// ThreadManager struct definition
pub struct ThreadManager {
    active_threads: Arc<Mutex<Vec<ThreadInfo>>>,
}

// ThreadInfo struct definition
#[derive(Debug, Clone, Serialize)]
pub struct ThreadInfo {
    name: String,
    latest_status: ThreadStatus,
    #[serde(skip)]
    shutdown_tx: Option<watch::Sender<()>>,
}

// Thread status enum definition
#[derive(Debug, Clone, Serialize, Copy)]
pub enum ThreadStatus {
    Running,
    Stopped,
    NotFound,
}

impl ThreadManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active_threads: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Spawn a new thread
    pub fn spawn<T>(&self, name: String, future: T)
    where
        T: std::future::Future<Output = ()> + Send + 'static,
    {
        let (shutdown_tx, mut shutdown_rx) = watch::channel(());

        let thread_info = ThreadInfo {
            name: name.clone(),
            latest_status: ThreadStatus::Running,
            shutdown_tx: Some(shutdown_tx),
        };

        let mut active_threads = self.active_threads.lock().unwrap();
        active_threads.push(thread_info.clone());

        let active_threads_clone = Arc::clone(&self.active_threads);

        tokio::spawn(async move {
            tokio::select! {
                _ = future => {},
                _ = shutdown_rx.changed() => {
                    // Handle the shutdown signal
                },
            }

            // Update the thread status to Stopped
            let mut active_threads = active_threads_clone.lock().unwrap();
            if let Some(thread) = active_threads.iter_mut().find(|t| t.name == name) {
                thread.latest_status = ThreadStatus::Stopped;
            }
        });
    }

    /// Get a single thread's status
    pub fn get_thread_status(&self, thread_name: String) -> ThreadStatus {
        let listan = self.get_active_threads();
        for item in listan.iter() {
            if item.name == thread_name {
                return item.latest_status;
            }
        }
        return ThreadStatus::NotFound;
    }

    /// Get all running threads
    pub fn get_active_threads(&self) -> Vec<ThreadInfo> {
        self.active_threads.lock().unwrap().clone()
    }

    /// Stop a thread
    pub fn stop_thread(&self, thread_name: &str) -> ThreadStatus {
        let mut active_threads = self.active_threads.lock().unwrap();
        if let Some(thread) = active_threads.iter_mut().find(|t| t.name == thread_name) {
            if let Some(shutdown_tx) = &thread.shutdown_tx {
                let _ = shutdown_tx.send(());
                thread.latest_status = ThreadStatus::Stopped;
                return ThreadStatus::Stopped;
            }
        }
        ThreadStatus::NotFound
    }
}
