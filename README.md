# option contracts

[![crates.io](https://meritbadge.herokuapp.com/hotpot-db)](https://crates.io/crates/optioncontracts)

written in rust - because rust.

Build options with the [Builder Pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).

```rust
let option_input = OptionBuilder::new()
    .kind(Type::Call)
    .direction(Direction::Long)
    .strike(10.0)
    .price(1.0)
    .finish();
```

Execute the option based on a current market price

```rust
let current_price = 20.0;
let result = execute_option(&option_input, current_price);

println!("{:#?}", result);
// 9.0
```

### Run Example

```bash
cargo run --example simple
```
