use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq)]
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
  stop:        Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Type(pub String, pub TypeLimit);

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeLimit {
  Int {
    min:     Option<u64>,
    max:     Option<u64>,
    default: Option<String>,
  },
  Float {
    min:     Option<f64>,
    max:     Option<f64>,
    default: Option<String>,
  },
  String {
    minLen:  Option<usize>,
    maxLen:  Option<usize>,
    default: Option<String>,
  }, // fixme: replace usize to u64
}
