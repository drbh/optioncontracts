#[derive(Debug, Clone)]
pub enum Type {
    Put,
    Call,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Long,
    Short,
}

#[derive(Debug)]
pub struct OptionBuilder {
    pub optiontype: Option<Type>,
    pub direction: Option<Direction>,
    pub strike: Option<f64>,
    pub price: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct OptionContract {
    pub optiontype: Type,
    pub direction: Direction,
    pub strike: f64,
    pub price: f64,
}

impl OptionBuilder {
    pub fn new() -> OptionBuilder {
        OptionBuilder {
            optiontype: None,
            direction: None,
            strike: None,
            price: None,
        }
    }

    pub fn kind(mut self, typ: Type) -> OptionBuilder {
        self.optiontype = Some(typ);
        self
    }

    pub fn direction(mut self, typ: Direction) -> OptionBuilder {
        self.direction = Some(typ);
        self
    }

    pub fn strike(mut self, value: f64) -> OptionBuilder {
        self.strike = Some(value);
        self
    }

    pub fn price(mut self, value: f64) -> OptionBuilder {
        self.price = Some(value);
        self
    }

    pub fn finish(self) -> OptionContract {
        OptionContract {
            optiontype: self.optiontype.unwrap(),
            direction: self.direction.unwrap(),
            strike: self.strike.unwrap(),
            price: self.price.unwrap(),
        }
    }
}

pub fn short_call(strike: f64, current_price: f64, cost: f64) -> f64 {
    match current_price > strike {
        true => current_price - strike + cost,
        false => cost,
    }
}

pub fn long_call(strike: f64, current_price: f64, cost: f64) -> f64 {
    match current_price > strike {
        true => current_price - strike - cost,
        false => -cost,
    }
}

pub fn short_put(strike: f64, current_price: f64, cost: f64) -> f64 {
    match current_price < strike {
        true => current_price - strike + cost,
        false => cost,
    }
}

pub fn long_put(strike: f64, current_price: f64, cost: f64) -> f64 {
    match current_price < strike {
        true => current_price - strike - cost,
        false => -cost,
    }
}

pub fn execute_option(option_input: &OptionContract, current_price: f64) -> f64 {
    match option_input.optiontype {
        Type::Call => match option_input.direction {
            Direction::Long => long_call(option_input.strike, current_price, option_input.price),
            Direction::Short => short_call(option_input.strike, current_price, option_input.price),
        },
        Type::Put => match option_input.direction {
            Direction::Long => long_put(option_input.strike, current_price, option_input.price),
            Direction::Short => short_put(option_input.strike, current_price, option_input.price),
        },
    }
}
