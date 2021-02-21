use std::io;
use std::convert::From;

#[derive(Debug)]
pub enum EditorError {
    IoError(io::Error),
    InvalidFile(String)
}

impl From<io::Error> for EditorError {
    fn from(e: io::Error) -> Self {
        EditorError::IoError(e)
    }
}