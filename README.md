# promptc-rs

## use block parser

```rust
use promptc::block::Unit;
let unit: Result<Unit, _> = Unit::from_str("...");
```

## use example

```sh
> cargo run --example exam abc
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `C:\Users\enter\projects\.cargo\targets\debug\examples\exam.exe abc`
Ok(Unit([Expr { normal_text: "abc", interp: None }]))
```
