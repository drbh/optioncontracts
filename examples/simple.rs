use optioncontracts::*;

fn main() {
    let option_input = OptionBuilder::new()
        .kind(Type::Call)
        .direction(Direction::Long)
        .strike(10.0)
        .price(1.0)
        .finish();

    println!("{:#?}", option_input);

    for i in 10..14 {
        let result = execute_option(&option_input, i as f64);

        println!("{:#?}", result);
    }
}
