use yahoo_finance_api as yahoo;
use time::OffsetDateTime;
use tokio;

mod models;
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect(&url).await.unwrap();
    println!("db connected, migrations applied");

    let provider = yahoo::YahooConnector::new().unwrap();
    let response = provider.get_latest_quotes("XEQT.TO", "1d").await.unwrap();
    let quote = response.last_quote().unwrap();
    let time = OffsetDateTime::from_unix_timestamp(quote.timestamp).unwrap();
    println!("At {} the price of XEQT was {}", time, quote.close);


}