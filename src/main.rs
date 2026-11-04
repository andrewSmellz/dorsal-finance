

use yahoo_finance_api as yahoo;




use crate::utils::balance::net_worth;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::connect(&url).await.expect("failed to connect to database");
    let provider = yahoo::YahooConnector::new().expect("failed to connect to yahoo");

    net_worth(&pool,&provider).await;

}
