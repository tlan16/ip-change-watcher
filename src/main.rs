mod ip_fetcher;
mod database;

use std::time::SystemTime;
// mod kafka;
use ip_fetcher::{fetch_ip_v4, fetch_ip_v6};
use database::entities;
use entities::{public_ips, prelude::PublicIps};
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait};
use tokio::join;
use chrono::prelude::{DateTime, Utc};

#[tokio::main]
async fn main() {
  // run_produce().await;

  let database = Database::connect("sqlite://src/database/database.sqlite?mode=rwc").await.unwrap();
  let maybe_new_public_ip = is_ip_changed(&database).await;
  if maybe_new_public_ip.is_none() {
    println!("No change in public ip");
    return;
  }
  let new_public_ip = maybe_new_public_ip.unwrap();
  
  on_ip_changed(&database, new_public_ip).await;
}

async fn is_ip_changed(database: &DatabaseConnection) -> Option<public_ips::ActiveModel> {
  let last_public_ip = get_last_public_ip(database).await;
  println!("Last public ip: {:?}", last_public_ip);
  
  if last_public_ip.is_none()  {
    return None;
  }

  let (ip_v4, ip_v6) = join!(fetch_ip_v4(), fetch_ip_v6());
  println!("public ipv4 address: {:?}", ip_v4);
  println!("public ipv6 address: {:?}", ip_v6);
  
  let old = last_public_ip.unwrap();
  if old.ip_v4 == ip_v4 && old.ip_v6 == ip_v6 {
    return None
  }
  
  Some(public_ips::ActiveModel {
    ip_v4: ActiveValue::Set(ip_v4),
    ip_v6: ActiveValue::Set(ip_v6),
    created_at: ActiveValue::Set(iso8601(SystemTime::now())),
  })
}

async fn on_ip_changed(database: &DatabaseConnection, new_public_ip: public_ips::ActiveModel) {

  PublicIps::insert(new_public_ip).exec(database).await.unwrap();
  let last_public_ip = get_last_public_ip(&database).await;
  println!("Inserted new public ip: {:?}", last_public_ip);
}

async fn get_last_public_ip(db: &DatabaseConnection) -> Option<public_ips::Model> {
  PublicIps::find().one(db).await.unwrap()
}

fn iso8601(st: std::time::SystemTime) -> String {
  let now: DateTime<Utc> = st.into();
  now.to_rfc3339()
}