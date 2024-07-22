use std::collections::HashMap;

pub async fn fetch_ip_v4() -> String {
  let resp = reqwest::get("https://api.ipify.org?format=json")
    .await.unwrap()
    .json::<HashMap<String, String>>()
    .await.unwrap();
  let ip_v4 = resp.get("ip").unwrap();
  ip_v4.to_string()
}

pub async fn fetch_ip_v6() -> String {
  let resp = reqwest::get("https://api64.ipify.org?format=json")
    .await.unwrap()
    .json::<HashMap<String, String>>()
    .await.unwrap();
  let ip_v6 = resp.get("ip").unwrap();
  ip_v6.to_string()
}