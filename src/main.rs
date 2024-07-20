#[allow(unused_imports)]
use cryptify;
use goldberg::goldberg_stmts;
#[tokio::main]

async fn main() {
  goldberg_stmts! {
    _main().await;
  }
}

async fn _main() {
  if let Some(ip) = public_ip::addr().await {
    println!("public ip address: {:?}", ip);
  } else {
    println!("couldn't get an IP address");
  }
}