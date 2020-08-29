use crate::cne::Elector;
use crate::cne::{make_search_url, Citizenship};
use crate::request::RequestError;
use crate::utils::is_valid_html_response;
use reqwest;
use std::time::Duration;

/// Creates a request to the server with a timeout of 10 seconds.
///
/// # Example
///
/// ```ignore
/// use libcne_ve::request::find;
/// use libcne_ve::cne::{Citizenship, Elector};
///
/// #[tokio::main]
/// async fn main() {
///   let elector_id: String = String::from("123123123");
///   let elector: Elector = libcne_ve::request::find(Citizenship::V, elector_id).await.unwrap();
///
///   println!("{:?}", elector);
/// }
/// ```
///
pub async fn find(citizenship: Citizenship, identity: String) -> Result<Elector, RequestError> {
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let url = make_search_url(citizenship, identity);
    let response = http_client.get(&url).send().await?.text().await?;

    if is_valid_html_response(&response) {
        return Ok(Elector::from(response));
    }

    Err(RequestError::new("Invalid document".into()))
}
