//! `model` defines the abstract model of the wiki. It consists of a [`Wiki`] struct associating
//! [`String`]-s to [`Page`]-s. Each [`Page`] tracks its unique parent, outgoing and incoming
//! links, and carries rendered HTML.

use std::collections::{HashMap, HashSet};

use sqlx::SqlitePool;
use paste::paste;

use crate::{
    db::{init_database, insert_graph},
    error::{Error, Result},
    files::{Files, strip_extension_from_filename},
    implement_accessors,
    parser::{Frontmatter, Html, ParsedPage, parse_raw_page},
};

#[derive(Debug, Clone)]
pub struct Wiki {
    pub pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub struct Page {
    pub content:  Html,
    pub parent:   Option<String>,
    pub outgoing: HashSet<String>,
    pub incoming: HashSet<String>,
    pub metadata: Option<Frontmatter>,
}

impl Page {
    implement_accessors!(
        parent: String,
        latex: bool,
        code: bool,
    );
}

impl Wiki {
    pub async fn init_from_files(files: Files) -> Result<Self> {
        let mut pages: HashMap<String, ParsedPage> = files
            .into_iter()
            .map(|(name, content)| {
                let page = parse_raw_page(&content)?;
                let id = strip_extension_from_filename(&name).ok_or(Error::Parse)?;
                Ok((id, page))
            })
            .collect::<Result<_>>()?;
        let valid_ids: HashSet<_> = pages.keys().cloned().collect();
        pages.values_mut().for_each(|page| {
            page.set_parent(page.parent().filter(|p| valid_ids.contains(p)));
            page.links = page
                .links
                .iter()
                .filter(|p| valid_ids.contains(*p))
                .cloned()
                .collect();
        });
        let pool = init_database().await;
        insert_graph(&pool, pages).await?;
        Ok(Self { pool })
    }
}
