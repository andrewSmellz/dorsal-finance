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

    // let your_account = Account::new("my account".to_string(), AccountKind::RRSP);
    // make_account(&pool, your_account).await.unwrap();

    // let my_account = get_account_by_name(&pool, "my account".to_string()).await.unwrap();
    // println!("{:?}",my_account);

    // println!("{:?}",get_all_accounts(&pool).await.unwrap());

    let account = get_account_by_name(&pool, "our account".into()).await.unwrap().unwrap();

    let txs = list_transactions(&pool, account.id).await.unwrap();
    println!("{:#?}",txs );

    let balance = account_balance(&pool, account.id).await;
    println!("ACCOUNT BALANCE IS {}",balance);

}
