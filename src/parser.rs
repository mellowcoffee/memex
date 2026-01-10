//! The module `parser` parses Markdown files into metadata extracted from frontmatter and rendered
//! HTML.

use gray_matter::{Matter, engine::YAML};
use pulldown_cmark::{Event, Options, Parser, Tag, html};
use serde::Deserialize;
use paste::paste;

use crate::error::Result;

macro_rules! implement_accessors {
    // Pattern: field_name : type
    ($($field:ident : $type:ty),* $(,)?) => {
        paste! {
            $(
                pub fn $field(&self) -> Option<$type> {
                    self.metadata.as_ref().and_then(|m| m.$field.clone())
                }

                pub fn [<set_ $field>](&mut self, new: Option<$type>) {
                    let meta = self.metadata.get_or_insert_with(Frontmatter::default);
                    meta.$field = new;
                }
            )*
        }
    };
}

#[derive(Deserialize, Clone)]
pub struct ParsedPage {
    pub html:   Html,
    pub links:  Vec<String>,
    pub metadata: Option<Frontmatter>,
}

impl ParsedPage {
    implement_accessors!(
        parent: String,
    );
}


#[derive(Deserialize, Clone, Default)]
pub struct Frontmatter {
    pub parent: Option<String>,
}

#[derive(Deserialize, Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Html(String);
impl From<String> for Html {
    fn from(string: String) -> Self {
        Html(string)
    }
}

impl std::fmt::Display for Html {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse_raw_page(raw: &str) -> Result<ParsedPage> {
    let (frontmatter, raw_content) = parse_frontmatter(raw)?;
    let (html, links) = parse_markdown(&raw_content);
    Ok(ParsedPage {
        html,
        links,
        metadata: frontmatter,
    })
}

fn parse_frontmatter(content: &str) -> Result<(Option<Frontmatter>, String)> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse::<Frontmatter>(content)?;
    let frontmatter = parsed.data;
    Ok((frontmatter, parsed.content))
}

fn parse_markdown(content: &str) -> (Html, Vec<String>) {
    let options = Options::empty();
    let parser = Parser::new_ext(content, options);

    let mut html = String::new();
    let mut links: Vec<String> = Vec::new();

    let iter = parser.map(|event| {
        if let Event::Start(Tag::Link { dest_url, .. }) = &event {
            if !dest_url.starts_with("http")
                && !dest_url.starts_with("mailto:")
                && !dest_url.starts_with('#')
            {
                links.push(dest_url.to_owned().to_string());
            }
        }
        event
    });

    html::push_html(&mut html, iter);
    (html.into(), links)
}
