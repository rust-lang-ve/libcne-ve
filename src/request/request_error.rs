use std::fmt;
use std::error::Error;
use reqwest::{Error as ReqwestError};

#[derive(Debug)]
pub struct RequestError {
  pub message: String,
}

impl From<ReqwestError> for RequestError {
  fn from(error: ReqwestError) -> Self {
    RequestError {
      message: error.status().unwrap().to_string()
    }
  }
}

impl fmt::Display for RequestError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!("The request returned {}", self.message))
  }
}

impl Error for RequestError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self)
  }
}
