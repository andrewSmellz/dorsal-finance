mod db;
mod models;
mod queries;

use tokio;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;


use models::{Account, AccountKind};
use queries::account::{get_account, get_all_accounts, make_account};

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

    let your_account = Account::new("our account".to_string(), AccountKind::Chequing);
    make_account(&pool, your_account).await.unwrap();

    let my_account = get_account(&pool, "my account".to_string()).await.unwrap();
    println!("{:?}",my_account);

    println!("{:?}",get_all_accounts(&pool).await.unwrap());
}
