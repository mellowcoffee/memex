use std::collections::HashMap;

use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool, Transaction};

use crate::{error::Result, model::Page, parser::ParsedPage};

const SCHEMA: &str = include_str!(".././migrations/schema.sql");

pub async fn init_database() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite://data.db").await.expect(
        "[ERROR] Could not connect to database. Ensure that data.db exists at the project root!",
    );
    let _res = sqlx::query(SCHEMA)
        .execute(&pool)
        .await
        .expect("[ERROR] Failed to create schema.");
    pool
}

pub async fn _insert_page(pool: &SqlitePool, id: String, page: ParsedPage) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO pages (id, content, parent_id) 
        VALUES (
            ?, 
            ?, 
            (SELECT id FROM pages WHERE id = ?)
        )
        ON CONFLICT(id) DO UPDATE SET
            content = excluded.content,
            parent_id = excluded.parent_id
    "#,
    )
    .bind(&id)
    .bind(page.html.to_string())
    .bind(page.parent().unwrap_or_default())
    .execute(pool)
    .await?;

    sqlx::query("DELETE FROM links WHERE source_id = ?")
        .bind(&id)
        .execute(pool)
        .await?;

    if !page.links.is_empty() {
        let mut qb: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO links (source_id, target_id) ");
        qb.push_values(page.links, |mut b, target| {
            b.push_bind(&id).push_bind(target);
        });
        qb.push(" ON CONFLICT DO NOTHING");

        let query = qb.build();
        query.execute(pool).await?;
    }

    Ok(())
}

async fn insert_page_with_tx(
    tx: &mut Transaction<'_, Sqlite>,
    id: String,
    page: ParsedPage,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO pages (id, content, parent_id) 
        VALUES (
            ?, 
            ?, 
            (SELECT id FROM pages WHERE id = ?)
        )
        ON CONFLICT(id) DO UPDATE SET
            content = excluded.content,
            parent_id = excluded.parent_id
    "#,
    )
    .bind(&id)
    .bind(page.html.to_string())
    .bind(page.parent().unwrap_or_default())
    .execute(&mut **tx)
    .await?;

    sqlx::query("DELETE FROM links WHERE source_id = ?")
        .bind(&id)
        .execute(&mut **tx)
        .await?;

    if !page.links.is_empty() {
        let mut qb: QueryBuilder<Sqlite> =
            QueryBuilder::new("INSERT INTO links (source_id, target_id) ");
        qb.push_values(page.links, |mut b, target| {
            b.push_bind(&id).push_bind(target);
        });
        qb.push(" ON CONFLICT DO NOTHING");

        let query = qb.build();
        query.execute(&mut **tx).await?;
    }

    Ok(())
}

pub async fn insert_graph(pool: &SqlitePool, pages: HashMap<String, ParsedPage>) -> Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("PRAGMA defer_foreign_keys = ON")
        .execute(&mut *tx)
        .await?;
    for (id, page) in pages {
        insert_page_with_tx(&mut tx, id, page).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn get_page(pool: &SqlitePool, id: &str) -> Result<Page> {
    let query = r#"
        SELECT 
            p.id, 
            p.content, 
            p.parent_id,
            (SELECT json_group_array(target_id) 
            FROM links 
            WHERE source_id = p.id) AS fwd_json,
            (SELECT json_group_array(source_id) 
            FROM links 
            WHERE target_id = p.id) AS bwd_json
        FROM pages p
        WHERE p.id = ?;
    "#;

    let row = sqlx::query(query).bind(id).fetch_one(pool).await?;

    let fwd_raw: String = row.try_get("fwd_json")?;
    let bwd_raw: String = row.try_get("bwd_json")?;

    Ok(Page {
        content:  row.try_get("content")?,
        parent:   row.try_get("parent_id")?,
        outgoing: serde_json::from_str(&fwd_raw).unwrap_or_default(),
        incoming: serde_json::from_str(&bwd_raw).unwrap_or_default(),
    })
}

pub async fn get_children_ids(pool: &SqlitePool, id: &str) -> Result<Vec<String>> {
    Ok(
        sqlx::query_scalar("SELECT id FROM pages WHERE parent_id = ?")
            .bind(id)
            .fetch_all(pool)
            .await?,
    )
}

pub async fn get_sibling_ids(pool: &SqlitePool, id: &str) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(
        r#"
        SELECT id FROM pages 
        WHERE parent_id IS (SELECT parent_id FROM pages WHERE id = ?) 
    "#,
    )
    .bind(id)
    .bind(id)
    .fetch_all(pool)
    .await?)
}

pub async fn get_uncle_ids(pool: &SqlitePool, id: &str) -> Result<Vec<String>> {
    Ok(sqlx::query_scalar(
        r#"
        SELECT uncle.id 
        FROM pages current
        JOIN pages parent ON current.parent_id = parent.id
        JOIN pages uncle ON uncle.parent_id IS parent.parent_id
        WHERE current.id = ?
    "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?)
}
