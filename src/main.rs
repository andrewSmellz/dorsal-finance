use dorsal_finance::{db, models::Currency, utils::balance::net_worth};
use yahoo_finance_api as yahoo;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&url).await.expect("failed to connect to database");
    let provider = yahoo::YahooConnector::new().expect("failed to connect to yahoo");

    net_worth(&pool,&provider,Currency::CAD).await.expect("networth failed");

}
