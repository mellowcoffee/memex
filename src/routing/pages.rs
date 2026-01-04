use askama::Template;
use axum::{
    Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};

use crate::{AppState, templates::Base};
use crate::templates;

pub fn routes(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/{url}", get(get_page_by_id))
        .with_state(state)
}

async fn get_page_by_id(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
) -> impl IntoResponse {
    let page = state.wiki.pages.get(&page_id.to_owned().into());
    match page {
        None => axum::response::Html(
            templates::NotFound
                .render()
                .expect("[ERROR] Askama could not render static HTML"),
        ),
        Some(page) => {
            let incoming = page.incoming.iter().cloned().collect::<Vec<_>>();
            let outgoing = page.outgoing.iter().cloned().collect::<Vec<_>>();
            let parents_parent = page
                .parent
                .clone()
                .and_then(|p| state.wiki.pages.get(&p).and_then(|p| p.parent.clone()));
            let parents_siblings = state
                .wiki
                .pages
                .iter()
                .filter(|(id, p)| {
                    p.parent.is_some() && p.parent == parents_parent
                        || Some(id) == page.parent.as_ref().as_ref()
                })
                .map(|(id, _p)| id)
                .cloned()
                .collect::<Vec<_>>();
            let siblings = state
                .wiki
                .pages
                .iter()
                .filter(|(_id, p)| p.parent == page.parent)
                .map(|(id, _p)| id)
                .cloned()
                .collect::<Vec<_>>();
            let children = state
                .wiki
                .pages
                .iter()
                .filter(|(_id, p)| p.parent == Some(page_id.to_owned().into()))
                .map(|(id, _p)| id)
                .cloned()
                .collect::<Vec<_>>();
            let base = Base {
                page_id: page_id.into(),
                parent: page.parent.to_owned(),
                content: page.content.to_owned(),
                incoming,
                outgoing,
                parents_siblings,
                siblings,
                children,
                has_latex: false,
            };
            axum::response::Html(base.render().expect("[ERROR] Askama failed to render."))
        }
    }
}
