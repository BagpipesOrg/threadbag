

#[derive(Debug)]
pub struct ThreadInfo {
    name: String,
    // Add more information about the thread if needed
}

#[derive(Debug)]
pub enum Command {
    Status { job: String },
    Start { job: String },
    Stop { job: String },
}