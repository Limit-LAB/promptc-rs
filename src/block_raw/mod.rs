use pest::Parser;

use self::parser::{Block, ParseError, Rule};
use crate::block_raw::parser::ParseFrom;

pub mod parser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Unit(pub Vec<Expr>);

impl std::str::FromStr for Unit {
  type Err = ParseError;
  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let mut pairs = Block::parse(Rule::unit, str)?;
    Ok(Unit::parse_from(pairs.next().unwrap()))
  }
}

#[derive(Debug, PartialEq, Eq, Hash)]

pub struct Expr {
  pub normal_text: String,
  pub interp:      Option<Interp>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Interp {
  String(String),
  Script(Script),
  Variable(String),
  ReservedQuota,
  ReservedReservedQuota,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Script {
  ease_mode:   bool,
  script_text: String,
}
