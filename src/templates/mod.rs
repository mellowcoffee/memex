use askama::Template;

use crate::parser::Html;

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound;

#[derive(Template, Debug)]
#[template(path = "base.html")]
pub struct Base {
    pub page_id:  String,
    pub parent:   Option<String>,
    pub content:  Html,
    pub incoming: Vec<String>,
    pub outgoing: Vec<String>,

    pub parents_siblings: Vec<String>,
    pub siblings:         Vec<String>,
    pub children:         Vec<String>,

    pub latex: bool,
    pub code:  bool,
}
