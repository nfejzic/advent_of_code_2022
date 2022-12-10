use std::{error::Error, fmt::Display};

use anyhow::Result;

#[derive(Debug)]
pub struct StringError {
    msg: String,
}

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl<T> From<T> for StringError
where
    T: Into<String>,
{
    fn from(input: T) -> Self {
        Self { msg: input.into() }
    }
}

impl Error for StringError {}

pub fn read_file() -> Result<String> {
    let mut args = std::env::args();

    let file_name = args
        .nth(1)
        .ok_or_else(|| StringError::from("Please provide file path."))?;

    std::fs::read_to_string(&file_name)
        .map_err(|_| StringError::from(format!("Could not open file {file_name}")))
        .map_err(anyhow::Error::new)
}
