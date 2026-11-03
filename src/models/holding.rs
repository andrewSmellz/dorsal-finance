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

    pub fn apply_buy(&mut self, quantity: Decimal, cost: Decimal){
        self.quantity+=quantity;
        self.cost_basis+=cost;
    }
    pub fn apply_sell(&mut self, quantity:Decimal)->Result<(), String>{
        if quantity > self.quantity {
            return Err(format!("cannot sell {} of {}, only {} held", quantity, self.ticker, self.quantity));
        }

        let avg_cost = self.cost_basis / self.quantity;
        self.quantity-=quantity;

        if self.quantity.is_zero() {
            self.cost_basis = Decimal::ZERO;
        } else {
            self.cost_basis -= avg_cost * quantity;
        }
        Ok(())
    }
}