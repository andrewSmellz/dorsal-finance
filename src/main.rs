mod db;
mod models;
mod queries;
mod utils;

use tokio;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;
use rust_decimal::dec;


use models::{Account, AccountKind, Transaction, TransactionKind};
use queries::account::{get_account_by_name, get_all_accounts, make_account};
use queries::transaction::{list_transactions};

use crate::models::AccountKind::Chequing;
use crate::models::TransactionKind::Deposit;
use crate::utils::balance::{account_balance, net_worth};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect(&url).await.unwrap();
    println!("db connected, migrations applied");

    // let provider = yahoo::YahooConnector::new().unwrap();
    // let response = provider.get_latest_quotes("XEQT.TO", "1d").await.unwrap();
    // let quote = response.last_quote().unwrap();
    // let time = OffsetDateTime::from_unix_timestamp(quote.timestamp).unwrap();
    // println!("At {} the price of XEQT was {}", time, quote.close);





    let networth = net_worth(&pool).await;
    println!("{}",networth);

}
