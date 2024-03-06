use std::time::Duration;
use tokio::time::sleep;
//use tokio_util::task::TaskTracker;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

// keep track of the threads named with the scenarioid
pub struct ThreadManager {
    active_threads: Arc<Mutex<Vec<ThreadInfo>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ThreadInfo {
    name: String,
    latest_status: status,
}

#[derive(Debug, Clone, Serialize, Copy)]
pub enum status {
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

    /// spawn new thread   
    pub fn spawn<T>(&self, name: String, future: T)
    where
        T: std::future::Future<Output = ()> + Send + 'static,
    {
        let thread_info = ThreadInfo {
            name: name.to_string(),
            latest_status: status::Running,
        };

        // Add the thread name to the active threads list.

        let mut active_threads = self.active_threads.lock().unwrap();
        active_threads.push(thread_info);

        let active_threads_clone = Arc::clone(&self.active_threads);

        tokio::spawn(async move {
            let result = future.await;

            // Remove the thread name from the active threads list.
            let mut active_threads = active_threads_clone.lock().unwrap();
            active_threads.retain(|t| t.name != name);

            result
        });
    }

    /// get a single threads statuss
    pub fn get_thread_status(&self, thread_name: String) -> status {
        let listan = self.get_active_threads();
        for item in listan.iter() {
            if item.name == thread_name {
                return item.latest_status;
            }
        }
        return status::NotFound;
    }

    // get all running threads
    pub fn get_active_threads(&self) -> Vec<ThreadInfo> {
        self.active_threads.lock().unwrap().clone()
    }
}
