mod ip_fetcher;
// mod kafka;

use {
  ip_fetcher::{fetch_ip_v4, fetch_ip_v6},
};

#[tokio::main]
async fn main() {
  // run_produce().await;

  println!("public ipv4 address: {:?}", fetch_ip_v4().await);
  println!("public ipv6 address: {:?}", fetch_ip_v6().await);

  // let db: DatabaseConnection = Database::connect("sqlite://database.sqlite?mode=rwc").await.unwrap();
}

