use rust_decimal::Decimal;
use crate::{error::{Error, Result}, models::{Currency, Money, Position}};
#[derive(Debug)]
pub struct Holding{
    pub position: Position,
    pub cost_basis: Money,
}

impl Holding{
    pub fn new(ticker: String, currency: Currency) -> Self {
        Holding {
            position: Position {ticker, quantity: Decimal::ZERO },
            cost_basis: Money::new(Decimal::ZERO, currency),
        }
    }

    pub fn apply_buy(&mut self, quantity: Decimal, cost: Money) -> Result<()> {
    if cost.currency != self.cost_basis.currency {
        return Err(Error::CurrencyMismatch {
            expected: self.cost_basis.currency,
            found: cost.currency,
        });
    }
    self.position.quantity += quantity;
    self.cost_basis.value += cost.value;
    Ok(())
}

    pub fn apply_sell(&mut self, quantity: Decimal) -> Result<()> {
        if quantity <= Decimal::ZERO {
            return Err(Error::InvalidAmount);
        }
        if quantity > self.position.quantity {
            return Err(Error::InsufficientShares {
                ticker: self.position.ticker.clone(),
                requested: quantity,
                held: self.position.quantity,
            });
        }

        let avg_cost = self.cost_basis.value / self.position.quantity;
        self.position.quantity -= quantity;

        if self.position.quantity.is_zero() {
            self.cost_basis.value = Decimal::ZERO;
        } else {
            self.cost_basis.value -= avg_cost * quantity;
        }
        Ok(())
    }
}