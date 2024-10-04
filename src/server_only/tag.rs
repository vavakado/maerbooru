use anyhow::{anyhow, Ok};
use regex::Regex;
use surrealdb::{sql::parse, Connection, Surreal};

use crate::models::tag::Tag;

fn is_snake_case(s: &str) -> bool {
    let re = Regex::new(r"^[a-z0-9():'_-]+([a-z0-9{}:'_-]+)*$").unwrap();
    re.is_match(s)
}

pub fn build_search_query(search: String) -> String {
    if search.is_empty() {
        return String::new();
    }

    let parts: Vec<&str> = search.split('*').collect();
    let parts_len = parts.len();

    if parts_len == 1 {
        // No wildcards, exact match
        return format!("WHERE name = '{}'", escape_string(search));
    }

    let mut conditions = Vec::new();

    if !parts[0].is_empty() {
        conditions.push(format!(
            "string::starts_with(name, '{}')",
            escape_string(parts[0].into())
        ));
    }

    if !parts[parts_len - 1].is_empty() {
        conditions.push(format!(
            "string::ends_with(name, '{}')",
            escape_string(parts[parts_len - 1].into())
        ));
    }

    for &part in &parts[1..parts_len - 1] {
        if !part.is_empty() {
            conditions.push(format!(
                "string::contains(name, '{}')",
                escape_string(part.into())
            ));
        }
    }

    if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    }
}

fn escape_string(s: String) -> String {
    s.replace('\'', "''")
}

pub async fn get_paginated_tags<C: surrealdb::Connection>(
    db: &Surreal<C>,
    page: u32,
    per_page: u32,
    search: Option<String>,
) -> Result<Vec<Tag>, anyhow::Error> {
    let offset = (page - 1) * per_page;
    let mut query = "SELECT * FROM tag".to_string();

    if let Some(search_term) = search {
        let where_clause = build_search_query(search_term);
        if !where_clause.is_empty() {
            query.push_str(&format!(" {}", where_clause));
        }
    }

    query.push_str(" ORDER BY use_count DESC LIMIT $limit START $offset");

    let tags: Vec<Tag> = db
        .query(&query)
        .bind(("limit", per_page))
        .bind(("offset", offset))
        .await?
        .take(0)?;

    Ok(tags)
}

async fn get_next_id<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<u64, anyhow::Error> {
    #[derive(serde::Deserialize)]
    struct IdCounter {
        last_id: i64,
    }

    let result: Option<IdCounter> = db
        .query("UPDATE id_counter SET last_id += 1 RETURN last_id")
        .await?
        .take(0)?;

    match result {
        Some(counter) => Ok(counter.last_id as u64),
        None => {
            // If no record exists, create one starting from 1
            let created: Option<IdCounter> = db
                .query("CREATE id_counter SET last_id = 1 RETURN last_id")
                .await?
                .take(0)?;
            match created {
                Some(counter) => Ok(counter.last_id as u64),
                None => Err(anyhow!("Failed to initialize id_counter")),
            }
        }
    }
}

pub async fn define_tag_table<T: Connection>(db: &surrealdb::Surreal<T>) -> anyhow::Result<()> {
    let schema = r#"
        DEFINE TABLE tag SCHEMAFULL;
        
        DEFINE FIELD custom_id ON TABLE tag TYPE number;
        DEFINE FIELD name ON TABLE tag TYPE string;
        DEFINE FIELD description ON TABLE tag TYPE string;
        DEFINE FIELD is_alias ON TABLE tag TYPE option<number>;
        DEFINE FIELD category ON TABLE tag TYPE number;
        DEFINE FIELD implications ON TABLE tag TYPE array;
        DEFINE FIELD use_count ON TABLE tag TYPE number;
        
        DEFINE INDEX custom_id ON TABLE tag FIELDS custom_id UNIQUE;
        DEFINE INDEX name_unique ON TABLE tag FIELDS name UNIQUE;
        "#;

    db.query(parse(schema)?).await?;

    Ok(())
}

pub async fn get_tag_by_name<T: Connection>(
    db: &surrealdb::Surreal<T>,
    name: String,
) -> Result<Option<Tag>, anyhow::Error> {
    let result: Option<Tag> = db
        .query("SELECT * FROM tag WHERE name = $name")
        .bind(("name", name))
        .await?
        .take(0)?;

    Ok(result)
}

pub async fn get_tag_by_id<C: surrealdb::Connection>(
    db: &Surreal<C>,
    custom_id: u64,
) -> Result<Option<Tag>, anyhow::Error> {
    let result: Option<Tag> = db
        .query("SELECT * FROM tag WHERE custom_id = $custom_id")
        .bind(("custom_id", custom_id))
        .await?
        .take(0)?;

    Ok(result)
}

pub async fn add_new_tag<C: surrealdb::Connection>(
    db: &Surreal<C>,
    tag: &Tag,
) -> Result<u64, anyhow::Error> {
    if !is_snake_case(&tag.name) {
        return Err(anyhow!("Tag name({}) must be in snake_case", &tag.name));
    }

    define_tag_table(db).await?;

    let created: Option<Tag> = db
        .create("tag")
        .content(Tag {
            custom_id: get_next_id(db).await?,
            ..tag.clone()
        })
        .await?;

    match created {
        Some(created_tag) => Ok(created_tag.custom_id),
        None => Err(anyhow!("failed to create tag")),
    }
}
