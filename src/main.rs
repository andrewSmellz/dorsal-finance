mod db;
mod models;
mod queries;
mod utils;

use tokio;
use yahoo_finance_api as yahoo;



use models::{Account, AccountKind, Transaction, TransactionKind};
use queries::account::{get_account_by_name, get_all_accounts, make_account};
use queries::transaction::{list_transactions};

use crate::utils::balance::{account_balance, net_worth};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect(&url).await.unwrap();
    println!("db connected, migrations applied");

    let provider = yahoo::YahooConnector::new().unwrap();

    net_worth(&pool).await;

}
