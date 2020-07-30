use crate::cne::{Citizenship, make_search_url};
use crate::request::RequestError;
use reqwest;

pub async fn find(citizenship: Citizenship, identity: String) -> Result<(), RequestError> {
  let http_client = reqwest::Client::builder()
    .build()
    .unwrap();

  let url = make_search_url(citizenship, identity);
  let response = http_client.get(&url).send().await?;

  println!("{:?}", response);

  Ok(())
}
