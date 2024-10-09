use surrealdb::{sql::parse, Connection, Surreal};

use crate::models::post::{Post, PostCreateError, PostSearchError};

/// search for posts by tags
pub async fn get_paginated_posts<C: surrealdb::Connection>(
    db: &Surreal<C>,
    page: u32,
    per_page: u32,
    search: Option<String>,
) -> Result<Vec<Post>, PostSearchError> {
    todo!();
}

/// find the post by it's unique id
pub async fn get_post_by_id<C: surrealdb::Connection>(
    db: &Surreal<C>,
    custom_id: u64,
) -> Result<Option<Post>, PostSearchError> {
    todo!();
}

pub async fn add_new_post<C: surrealdb::Connection>(
    db: &Surreal<C>,
    post: &Post,
) -> Result<Option<Post>, PostCreateError> {
    todo!();
}

pub async fn remove_post<C: surrealdb::Connection>(
    db: &Surreal<C>,
    custom_id: u64,
) -> Result<(), anyhow::Error> {
    todo!();
}

async fn get_next_id<C: surrealdb::Connection>(db: &Surreal<C>) -> Result<u64, anyhow::Error> {
    todo!();
}

pub async fn define_post_table<C: Connection>(db: &Surreal<C>) -> Result<(), anyhow::Error> {
    let schema = r#"
        DEFINE TABLE post SCHEMAFULL;
        
        DEFINE FIELD custom_id ON post TYPE number;
        DEFINE FIELD image_height ON post TYPE number;
        DEFINE FIELD image_width ON post TYPE number;
        DEFINE FIELD mime ON post TYPE string;
        DEFINE FIELD post_type ON post TYPE string ASSERT $value IN ['Image', 'Video'];
        DEFINE FIELD safety ON post TYPE string ASSERT $value IN ['Safe', 'Sketchy', 'Unsafe'];
        DEFINE FIELD sha256_hash ON post TYPE array;
        DEFINE FIELD md5_hase ON post TYPE array;
        DEFINE FIELD source ON post TYPE string;
        DEFINE FIELD uploaded ON post TYPE datetime;
        DEFINE FIELD uploader_id ON post TYPE number;
        DEFINE FIELD tags ON post TYPE array;
        DEFINE FIELD relations ON post TYPE array;
        
        DEFINE INDEX post_custom_id ON post FIELDS custom_id UNIQUE;
        "#;

    db.query(parse(schema)?).await?;

    Ok(())
}

pub fn build_post_search_query(search: String) -> String {
    todo!();
}
