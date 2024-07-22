mod ip_fetcher;
mod database;
// mod kafka;
use ip_fetcher::{fetch_ip_v4, fetch_ip_v6};
use database::entities;
use entities::{public_ips, prelude::PublicIps};
use sea_orm::{Database, DatabaseConnection, EntityTrait};

#[tokio::main]
async fn main() {
  // run_produce().await;

  println!("public ipv4 address: {:?}", fetch_ip_v4().await);
  println!("public ipv6 address: {:?}", fetch_ip_v6().await);

  let database = Database::connect("sqlite://src/database/database.sqlite?mode=rwc").await.unwrap();
  let last_public_ip = get_last_public_ip(&database).await;
  println!("Last public ip: {:?}", last_public_ip);
}

async fn get_last_public_ip(db: &DatabaseConnection) -> Option<public_ips::Model> {
  PublicIps::find().one(db).await.unwrap()
}