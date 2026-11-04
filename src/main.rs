mod db;
mod models;
mod queries;
mod utils;
mod error;

use yahoo_finance_api as yahoo;




use crate::utils::balance::net_worth;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect(&url).await.unwrap();
    println!("db connected, migrations applied");

    let provider = yahoo::YahooConnector::new().unwrap();

    net_worth(&pool,&provider).await;

}
