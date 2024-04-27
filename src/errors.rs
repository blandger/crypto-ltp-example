use std::fmt::Display;

/// Common error type while accessing remote site
pub type ClientResult<T> = Result<T, Error>;

/// Errors from library code
#[derive(Debug, Clone)]
pub enum Error {
    /// Can't connect to remote host
    Connect(String),
    /// Response contains business error
    IncorrectResponse(String),
    /// Can't fetch data
    FetchFailed(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Connect(err) => {
                write!(f, "Connect is failed: {}", err)
            }
            Error::IncorrectResponse(err) => {
                write!(f, "Response with error: {}", err)
            }
            Error::FetchFailed(err) => {
                write!(f, "Fetching is failed: {}", err)
            }
        }
    }
}
