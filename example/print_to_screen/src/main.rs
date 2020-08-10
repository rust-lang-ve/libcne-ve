use libcne::request::find;
use libcne::cne::{Citizenship, Elector};

#[tokio::main]
async fn main() {
  let elector: Elector = libcne::request::find(Citizenship::V, "8987677".to_string()).await.unwrap();

  println!("{:?}", elector);
}
