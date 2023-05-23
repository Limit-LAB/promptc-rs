fn main() {
  use std::str::FromStr;

  use promptc::block::Unit;
  let unit: Result<Unit, _> = Unit::from_str("{%");
  println!("{:?}", unit);
}
