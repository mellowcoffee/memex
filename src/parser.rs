//! The module `parser` parses Markdown files into metadata extracted from frontmatter and rendered
//! HTML.

use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;

use crate::{error::Result, model::PageId};

#[derive(Deserialize, Clone)]
pub struct ParsedPage {
    pub frontmatter: Option<Frontmatter>,
    pub html:        Html,
}

#[derive(Deserialize, Clone)]
pub struct Frontmatter {
    pub parent: Option<PageId>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Html(String);
impl From<String> for Html {
    fn from(string: String) -> Self {
        Html(string)
    }
}

pub fn parse_raw_page(raw: &str) -> Result<ParsedPage> {
    let (frontmatter, raw_content) = parse_frontmatter(raw)?;
    let html = parse_markdown(&raw_content);
    Ok(ParsedPage { frontmatter, html })
}

fn parse_frontmatter(content: &str) -> Result<(Option<Frontmatter>, String)> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse::<Frontmatter>(content)?;
    let frontmatter = parsed.data;
    Ok((frontmatter, parsed.content))
}

fn parse_markdown(content: &str) -> Html {
    let options = Options::empty();
    let parser = Parser::new_ext(content, options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html.into()
}
