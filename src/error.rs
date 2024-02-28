// Threadbags error class, goal is to move all error handling here //flipchan
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::error::Error),
    /// Could not write to database
    DBwritefail,
    /// Anyhow error wrapper
    Anyhow(anyhow::Error),
    /// Could not find entry in database
    NoEntryInDb,
    DecodeProblem,
}

impl From<anyhow::Error> for Error {
    fn from(src: anyhow::Error) -> Error {
        Error::Anyhow(src)
    }
}
