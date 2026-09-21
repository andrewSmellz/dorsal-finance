use rust_decimal::Decimal;



#[derive(Debug)]
pub struct Holding{
    name: String,
    ticker: String,
    quantity: Decimal,
    cost_basis: Decimal,
}

impl Holding{
    pub fn new(name: String, ticker: String, quantity: Decimal, value: Decimal) -> Self {
        Holding {
            name,
            ticker,
            quantity,
            cost_basis,
        }
    }
}