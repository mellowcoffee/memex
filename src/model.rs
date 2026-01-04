//! `model` defines the abstract model of the wiki. It consists of a [`Wiki`] struct associating
//! [`PageId`]-s to [`Page`]-s. Each [`Page`] tracks its unique parent, outgoing and incoming
//! links, and carries rendered HTML.

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    files::{Files, strip_extension_from_filename},
    parser::{Html, parse_raw_page},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct PageId(String);
impl From<String> for PageId {
    fn from(string: String) -> Self {
        Self(string)
    }
}

impl std::fmt::Display for PageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Wiki {
    pub pages: HashMap<PageId, Page>,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub content:  Html,
    pub parent:   Option<PageId>,
    pub outgoing: HashSet<PageId>,
    pub incoming: HashSet<PageId>,
}

impl Wiki {
    pub fn from_files(files: Files) -> Result<Self> {
        // Pass 1: parse content, construct vertices
        let mut pages: HashMap<PageId, Page> = files
            .into_iter()
            .map(|(name, content)| {
                let parsed = parse_raw_page(&content)?;
                let id = strip_extension_from_filename(&name)
                    .ok_or(Error::Parse)?
                    .into();
                let parent = parsed.frontmatter.and_then(|f| f.parent);
                let outgoing = HashSet::from_iter(parsed.links);
                let page = Page {
                    content: parsed.html,
                    parent,
                    outgoing,
                    incoming: HashSet::new(),
                };
                Ok((id, page))
            })
            .collect::<Result<_>>()?;

        // Pass 2: validate links
        let valid_ids: HashSet<_> = pages.keys().cloned().collect();
        pages.values_mut().for_each(|page| {
            page.parent = page.parent.take().filter(|p| valid_ids.contains(p));
            page.outgoing = page
                .outgoing
                .iter()
                .filter(|p| valid_ids.contains(p))
                .cloned()
                .collect();
        });

        // Pass 3: extract backlinks and parent links
        let backlinks: Vec<_> = pages
            .iter()
            .flat_map(|(from, page)| {
                let parent = page.parent.iter();
                let outgoing = page.outgoing.iter();
                parent
                    .chain(outgoing)
                    .map(move |to| (from.to_owned(), to.to_owned()))
            })
            .collect();

        // Pass 4: construct edges
        backlinks.into_iter().for_each(|(from, to)| {
            pages
                .get_mut(&to)
                .map(|page| page.incoming.insert(from.to_owned()));
        });

        let wiki = Wiki { pages };
        Ok(wiki)
    }
}
