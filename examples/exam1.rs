
fn main() {
  use promptc::block::Unit;
  use std::str::FromStr;
  let unit: Result<Unit, _> = Unit::from_str("{%Q%}");
  println!("{:?}", unit);
}