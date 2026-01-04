use askama::Template;

use crate::{model::PageId, parser::Html};

#[derive(Template)]
#[template(path = "404.html")]
pub struct NotFound;

#[derive(Template, Debug)]
#[template(path = "base.html")]
pub struct Base {
    pub page_id:  PageId,
    pub parent:   Option<PageId>,
    pub content:  Html,
    pub incoming: Vec<PageId>,
    pub outgoing: Vec<PageId>,

    pub parents_siblings: Vec<PageId>,
    pub siblings:         Vec<PageId>,
    pub children:         Vec<PageId>,

    pub has_latex: bool,
}
