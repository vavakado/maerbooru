use leptos::*;

use crate::models::tag::Tag;

#[server(GetPaginatedTags, "/api")]
pub async fn get_paginated_tags(
    page: u32,
    per_page: u32,
    search: Option<String>,
) -> Result<Vec<Tag>, ServerFnError> {
    use crate::server_only::db::get_db_connection;
    let db = get_db_connection().await?;

    let tags = crate::server_only::tag::get_paginated_tags(&db, page, per_page, search)
        .await
        .unwrap();

    Ok(tags)
}

#[server(AddNewTag, "/api")]
pub async fn add_new_tag(name: String) -> Result<u64, ServerFnError> {
    let db = crate::server_only::db::get_db_connection().await?;

    let new_tag = Tag {
        custom_id: 0, // This will be replaced by the database
        name,
        use_count: 0,
        description: String::new(),
        is_alias: None,
        category: 0,
        implications: vec![],
    };

    // Use the add_new_tag function we created earlier
    match crate::server_only::tag::add_new_tag(&db, &new_tag).await {
        Ok(custom_id) => Ok(custom_id),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
