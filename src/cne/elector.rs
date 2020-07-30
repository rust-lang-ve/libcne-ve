use crate::cne::Citizenship;
use serde::{Serialize, Deserialize};

/// The actor for the information provided by the CNE
/// website is represented as an `Elector`
#[derive(Debug, Serialize, Deserialize)]
pub struct Elector {
  pub citizenship: Citizenship,
  pub identity: String,
  pub name: String,
  pub state: String,
  pub town: String,
  pub parish: String,
  pub voting_center: String,
  pub address: String,
}

impl From<String> for Elector {
  fn from(html_string: String) -> Self {
    todo!()
  }
}
