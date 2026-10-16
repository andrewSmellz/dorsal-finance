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
use queries::transaction::{make_transaction, list_transactions};

use crate::models::AccountKind::Chequing;
use crate::models::TransactionKind::Deposit;
use crate::utils::balance::account_balance;

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





    let our_account = get_account_by_name(&pool, "our account".into()).await.unwrap().unwrap();
    let your_account = get_account_by_name(&pool, "your account".into()).await.unwrap().unwrap();

    let mut balance = account_balance(&pool, our_account.id).await;
    println!("OUR ACCOUNT BALANCE IS {}",balance);

    balance = account_balance(&pool, your_account.id).await;
    println!("YOUR ACCOUNT BALANCE IS {}",balance);

    queries::transaction::transfer(&pool, our_account.id, your_account.id,dec!(40)).await.unwrap();
    println!("transfer complete of 40");

    balance = account_balance(&pool, our_account.id).await;
    println!("OUR ACCOUNT BALANCE IS {}",balance);

    balance = account_balance(&pool, your_account.id).await;
    println!("YOUR ACCOUNT BALANCE IS {}",balance);

}
