use rust_decimal::Decimal;
use uuid::Uuid;


#[derive(Debug)]
pub struct Holding{
    pub ticker: String,
    pub quantity: Decimal,
    pub cost_basis: Decimal,
}

impl Holding{
    pub fn new(ticker: String, quantity: Decimal, cost_basis: Decimal) -> Self {
        Holding {
            ticker,
            quantity,
            cost_basis,
        }
    }
}