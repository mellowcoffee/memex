use askama::Template;
use axum::{
    extract::{Path, State}, response::{IntoResponse, Redirect}, routing::get, Router
};

use crate::{
    AppState,
    db::{get_children_ids, get_page, get_sibling_ids, get_uncle_ids},
    templates::Base,
};
use crate::templates;

pub fn routes(State(state): State<AppState>) -> Router {
    Router::new()
        .route("/", get(async || Redirect::permanent("/index")))
        .route("/{url}", get(get_page_by_id))
        .with_state(state)
}

async fn get_page_by_id(
    State(state): State<AppState>,
    Path(page_id): Path<String>,
) -> impl IntoResponse {
    let page = get_page(&state.wiki.pool, &page_id).await;
    match page {
        Err(_) => axum::response::Html(
            templates::NotFound
                .render()
                .expect("[ERROR] Askama could not render static HTML"),
        ),
        Ok(page) => {
            let incoming = page.incoming.iter().cloned().collect::<Vec<_>>();
            let outgoing = page.outgoing.iter().cloned().collect::<Vec<_>>();
            let parents_siblings = get_uncle_ids(&state.wiki.pool, &page_id)
                .await
                .unwrap_or_default();
            let siblings = get_sibling_ids(&state.wiki.pool, &page_id)
                .await
                .unwrap_or_default();
            let children = get_children_ids(&state.wiki.pool, &page_id)
                .await
                .unwrap_or_default();
            let base = Base {
                page_id,
                parent: page.parent.clone(),
                content: page.content.clone(),
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
