use std::fmt;

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    /// serde error
    SerdeJson(serde_json::error::Error),
    /// Could not write to database
    DBwritefail,
    /// Anyhow error wrapper
    Anyhow(anyhow::Error),
    /// Could not find entry in database
    NoEntryInDb,
    DecodeProblem,
    /// the source chain does not have support for sending to this destination chain
    UnsupportedDestinationChain,

    /// Subxt could not draft tx error
    Subxt(subxt::Error),

    /// Could not extract data from pill node
    PillDataError,

    /// Can not fetch webhook data
    CantFetchWebhook,

    /// Error parsing scenario
    ScenarioParseError,

    /// Invalid scenarioid
    ScenarioIdNotFound,
    /// polodb database problem
    Polodb(polodb_core::Error),

    /// Invalid destination chain
    InvalidDestinationChain,
    /// Invalid chain selected
    InvalidChainOption,

    /// problem parsing the uuid from the webhook
    CouldNotFindWebhookData,
    /// problems making a http request
    HTTPRequestProblem(reqwest::Error),

    // reqwest thirdparty lib error
    ReqwestError(reqwest::Error),

    Io(String),
    FileIOerror(std::io::Error),
    Stderror(String),
    // error in std::error
    STDLIBerror,
    /// error getting block events
    ErrorEvent,
    /// Could not find event in the latest blocks
    EventNotFound,
    /// Could not find the storage item
    StorageItemNotFound,
    /// hex crate error
    Hex(hex::FromHexError),
    MaxConnectionAttemptsExceeded,
    /// connection to endpoint closed
    ConnectionClosed,
    /// could not get the requested block
    CouldNotGetBlock,
    /// Could not get next item in async loop
    AsyncNext,
    /// Sled database error
    Sled(sled::Error),
    /// Error converting string to u8
    Utf8String(std::string::FromUtf8Error),
}

impl From<anyhow::Error> for Error {
    fn from(src: anyhow::Error) -> Error {
        Error::Anyhow(src)
    }
}

impl From<reqwest::Error> for Error {
    fn from(src: reqwest::Error) -> Error {
        Error::HTTPRequestProblem(src)
    }
}

impl From<sled::Error> for Error {
    fn from(src: sled::Error) -> Error {
        Error::Sled(src)
    }
}

impl From<polodb_core::Error> for Error {
    fn from(src: polodb_core::Error) -> Error {
        Error::Polodb(src)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(src: std::string::FromUtf8Error) -> Error {
        Error::Utf8String(src)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SerdeJson(e) => Some(e),
            Error::Anyhow(e) => Some(e.as_ref()),
            Error::Subxt(e) => Some(e),
            Error::Polodb(e) => Some(e),
            Error::HTTPRequestProblem(e) => Some(e),
            Error::ReqwestError(e) => Some(e),
            Error::FileIOerror(e) => Some(e),
            Error::Sled(e) => Some(e),
            Error::Hex(e) => Some(e),
            Error::Utf8String(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
            Error::DBwritefail => write!(f, "Could not write to database"),
            Error::Anyhow(e) => write!(f, "Anyhow error: {}", e),
            Error::NoEntryInDb => write!(f, "Could not find entry in database"),
            Error::DecodeProblem => write!(f, "Decode problem"),
            Error::UnsupportedDestinationChain => write!(f, "Unsupported destination chain"),
            Error::Subxt(e) => write!(f, "Subxt error: {}", e),
            Error::PillDataError => write!(f, "Could not extract data from pill node"),
            Error::CantFetchWebhook => write!(f, "Cannot fetch webhook data"),
            Error::ScenarioParseError => write!(f, "Error parsing scenario"),
            Error::ScenarioIdNotFound => write!(f, "Invalid scenario ID"),
            Error::Polodb(e) => write!(f, "PoloDB error: {}", e),
            Error::InvalidDestinationChain => write!(f, "Invalid destination chain"),
            Error::InvalidChainOption => write!(f, "Invalid chain selected"),
            Error::CouldNotFindWebhookData => write!(f, "Could not parse webhook UUID"),
            Error::HTTPRequestProblem(e) => write!(f, "HTTP request problem: {}", e),
            Error::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            Error::Io(msg) => write!(f, "IO error: {}", msg),
            Error::FileIOerror(e) => write!(f, "File IO error: {}", e),
            Error::Stderror(msg) => write!(f, "Standard error: {}", msg),
            Error::STDLIBerror => write!(f, "Standard library error"),
            Error::ErrorEvent => write!(f, "Error getting block events"),
            Error::EventNotFound => write!(f, "Could not find event in latest blocks"),
            Error::StorageItemNotFound => write!(f, "Could not find storage item"),
            Error::Hex(e) => write!(f, "Hex conversion error: {}", e),
            Error::MaxConnectionAttemptsExceeded => write!(f, "Max connection attempts exceeded"),
            Error::ConnectionClosed => write!(f, "Connection to endpoint closed"),
            Error::CouldNotGetBlock => write!(f, "Could not get requested block"),
            Error::AsyncNext => write!(f, "Could not get next item in async loop"),
            Error::Sled(e) => write!(f, "Sled database error: {}", e),
            Error::Utf8String(e) => write!(f, "UTF-8 string conversion error: {}", e),
        }
    }
}

impl From<subxt::Error> for Error {
    fn from(src: subxt::Error) -> Error {
        Error::Subxt(src)
    }
}
