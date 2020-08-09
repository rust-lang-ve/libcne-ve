use regex::RegexSet;

pub fn is_valid_html_response(html_string: &str) -> bool {
  let reg = RegexSet::new(&[
    r"Cédula",
    r"Nombre",
    r"Estado",
    r"Municipio",
    r"Parroquia",
    r"Centro",
    r"Dirección",
  ])
  .unwrap();

  let matches: Vec<_> = reg.matches(html_string).into_iter().collect();
  matches.len() == 7
}
