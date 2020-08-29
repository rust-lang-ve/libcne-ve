/// **libcne-ve** is a library that fetches data from a public endpoint in the CNE website and deserializes its contents.
///
/// Basically this crate, makes a request to the endpoint available in the site
/// used to fetch date about where a given ID (CID) belongs as voting center, scraps the
/// HTML response into an `Elector` `struct` and returns it.
///
/// # Example
///
/// The following sample is available in the `libcne-ve/example` directory:
///
/// ```ignore
/// use libcne_ve::request::find;
/// use libcne_ve::cne::{Citizenship, Elector};
///
/// #[tokio::main]
/// async fn main() {
///   let elector_id: String = String::from("123123123");
///   let elector: Elector = find(Citizenship::V, elector_id).await.unwrap();
///
///   println!("{:?}", elector);
/// }
/// ```
pub mod cne;
pub mod request;
pub mod utils;
