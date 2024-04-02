#[derive(Debug)]
pub struct ThreadInfo {
    name: String,
    // Add more information about the thread if needed
}

#[derive(Debug)]
pub enum Command {
    Status { scenario_id: String },
    Start { scenario_id: String, delay: u64 },
    Stop { scenario_id: String },
}
