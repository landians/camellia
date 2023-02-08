
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Erorr happened: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Service: {0} not found")]
    ServiceNotFound(String),

    #[error("Method: {0} not found")]
    MethodNotFound(String),
}