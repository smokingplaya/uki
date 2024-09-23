use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Argument {
  pub name: String,
  pub default: Option<String>
}