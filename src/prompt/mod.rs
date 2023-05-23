use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Prompt {
  pub project: Option<String>,
  pub author:  Option<String>,
  pub license: Option<String>,
  pub version: Option<String>,
  pub conf:    Option<Config>,
  pub vars:    Option<Vec<Type>>,
  pub prompts: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Config {
  provider:    Option<String>,
  model:       Option<String>,
  temperature: Option<String>,
  stop:        (String, String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub struct Type {}
