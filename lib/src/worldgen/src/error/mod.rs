use derive_more::{Display, Error};
// use rand::rngs::adapter::ReadError;

#[derive(Debug, Display, Error)]
pub enum Error {
    Bitmask { message: String },
    FileSystem { message: String },
    Json { message: String },
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::FileSystem {
            message: e.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json {
            message: e.to_string(),
        }
    }
}
