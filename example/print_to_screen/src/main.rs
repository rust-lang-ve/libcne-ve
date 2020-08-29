use libcne_ve::request::find;
use libcne_ve::cne::{Citizenship, Elector};

#[tokio::main]
async fn main() {
  let elector: Elector = libcne_ve::request::find(Citizenship::V, "8987677".to_string()).await.unwrap();

  println!("{:?}", elector);
}
