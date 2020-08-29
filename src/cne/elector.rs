use crate::cne::Citizenship;
use scraper::{element_ref::ElementRef, html::Select, Html, Selector};
use serde::{Deserialize, Serialize};

fn select_field<'a>(document: &'a mut Select) -> scraper::element_ref::ElementRef<'a> {
    let selector = Selector::parse("tr td:last-child").unwrap();
    document.next().unwrap().select(&selector).next().unwrap()
}

fn select_bold_text<'a>(field: &'a mut ElementRef<'a>) -> scraper::element_ref::ElementRef<'a> {
    let selector = Selector::parse("b").unwrap();
    field.select(&selector).next().unwrap()
}

fn select_font_tag<'a>(field: &'a mut ElementRef<'a>) -> scraper::element_ref::ElementRef<'a> {
    let selector = Selector::parse("font").unwrap();
    field.select(&selector).next().unwrap()
}

/// The actor for the information provided by the CNE
/// website is represented as an `Elector`
#[derive(Debug, Serialize, Deserialize)]
pub struct Elector {
    pub citizenship: Citizenship,
    pub identity: String,
    pub name: String,
    pub state: String,
    pub town: String,
    pub parish: String,
    pub voting_center: String,
    pub address: String,
}

impl From<String> for Elector {
    fn from(html_string: String) -> Self {
        let document = Html::parse_fragment(&html_string);
        let selector = Selector::parse("table tbody tr td table tbody tr:last-child td table tbody tr:nth-child(2) td table:first-child tbody tr").unwrap();
        let mut selection = document.select(&selector);

        let citizenship = select_field(&mut selection).inner_html();

        Elector {
            citizenship: match citizenship.chars().next().unwrap() {
                'E' => Citizenship::E,
                _ => Citizenship::V,
            },
            identity: citizenship
                .chars()
                .skip(2)
                .take(citizenship.len() - 2)
                .collect(),
            name: select_bold_text(&mut select_field(&mut selection)).inner_html(),
            state: select_field(&mut selection).inner_html(),
            town: select_field(&mut selection).inner_html(),
            parish: select_field(&mut selection).inner_html(),
            voting_center: select_font_tag(&mut select_field(&mut selection)).inner_html(),
            address: select_font_tag(&mut select_field(&mut selection)).inner_html(),
        }
    }
}
