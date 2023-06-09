use std::str::FromStr;

fn main() {
    use promptc::block_raw::Unit;
    let mut args = std::env::args();
    args.next().unwrap();
    for i in args {
        let unit: Result<Unit, _> = Unit::from_str(&i);
        println!("{:?}", unit);
    }
}
