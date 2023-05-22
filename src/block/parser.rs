// use std::assert_matches::debug_assert_matches;

use pest::{error::Error, iterators::Pair};
use pest_derive::Parser;

use super::{Expr, Interp, Script, Unit};

#[derive(Parser)]
#[grammar = "./block/grammar.pest"]
pub struct Block {}

pub type ParseError = Error<Rule>;

pub trait ParseFrom
where
  Self: std::marker::Sized,
{
  fn parse_from(pair: Pair<Rule>) -> Self;
}

impl ParseFrom for Unit {
  fn parse_from(pair: Pair<Rule>) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::unit);
    Self(
      pair
        .into_inner()
        .filter_map(|x| match x.as_rule() {
          Rule::expr => Some(Expr::parse_from(x)),
          Rule::EOI => None,
          _ => unreachable!(),
        })
        .collect(),
    )
  }
}

impl ParseFrom for Expr {
  fn parse_from(pair: Pair<Rule>) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::expr);
    let mut pairs = pair.into_inner();
    let normal_text = pairs.next().unwrap().as_str().to_string();
    let interp = pairs.next().map(Interp::parse_from);
    Self {
      normal_text,
      interp,
    }
  }
}

impl ParseFrom for Interp {
  fn parse_from(pair: Pair<Rule>) -> Self {
    debug_assert_eq!(pair.as_rule(), Rule::interp);
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
      Rule::reserved_quota => Self::ReservedQuota,
      Rule::easy_script => Self::Script(script_parse_from(pair, true)),
      Rule::script => Self::Script(script_parse_from(pair, false)),
      Rule::string => Self::String(pair.into_inner().next().unwrap().as_str().to_string()),
      Rule::variable => Self::Variable(pair.into_inner().next().unwrap().as_str().to_string()),
      _ => unreachable!(),
    }
  }
}

#[inline]
fn script_parse_from(pair: Pair<Rule>, ease_mode: bool) -> Script {
  // debug_assert_matches!(pair.as_rule(), Rule::script | Rule::easy_script);  // debug_assert_matches require unstable
  Script {
    ease_mode,
    script_text: pair.into_inner().next().unwrap().as_str().to_string(),
  }
}
