use rust_decimal::Decimal;
use uuid::Uuid;


#[derive(Debug)]
pub struct Holding{
    pub name: String,
    pub ticker: String,
    pub quantity: Decimal,
    pub cost_basis: Decimal,
    pub account_id: Uuid,
}

impl Holding{
    pub fn new(name: String, ticker: String, quantity: Decimal, cost_basis: Decimal, account_id: Uuid) -> Self {
        Holding {
            name,
            ticker,
            quantity,
            cost_basis,
            account_id
        }
    }
}