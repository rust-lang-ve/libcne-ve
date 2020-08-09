use crate::cne::Citizenship;

/// The base URL for the CNE website
pub const BASE_URL: &str = "http://www.cne.gob.ve/web/registro_electoral/ce.php";

/// Creates a URL to send the search request given
/// the `Citizenship` and the `identity`
///
/// # Example
///
/// ```rust
/// use libcne::cne::{Citizenship, make_search_url};
///
/// let url = make_search_url(Citizenship::V, String::from("000"));
/// let want = "http://www.cne.gob.ve/web/registro_electoral/ce.php?nacionalidad=V&cedula=000".to_string();
///
/// assert_eq!(url, want);
/// ```
pub fn make_search_url(citizenship: Citizenship, indentity: String) -> String {
  format!(
    "{}?nacionalidad={}&cedula={}",
    BASE_URL,
    citizenship.to_string(),
    indentity
  )
}
