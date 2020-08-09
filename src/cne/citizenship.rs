use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::string::ToString;

/// Enumerates posible citizenship statuses
/// such as V (Venezuelan) and E (Foreigner)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Citizenship {
  V,
  E,
}

pub struct ParseCitizenshipError {
  pub message: String,
}

impl FromStr for Citizenship {
  type Err = ParseCitizenshipError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "V" => Ok(Citizenship::V),
      "E" => Ok(Citizenship::E),
      _ => Err(ParseCitizenshipError {
        message: format!(
          "Invalid citizen value provided, {}. Valid values are either V and E.",
          s
        ),
      }),
    }
  }
}

impl ToString for Citizenship {
  fn to_string(&self) -> String {
    match self {
      Citizenship::V => String::from("V"),
      Citizenship::E => String::from("E"),
    }
  }
}
