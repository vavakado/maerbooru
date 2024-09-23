use leptos::*;

use crate::schemes::tag::TagData;

#[server]
pub async fn get_tag_by_id(id: String) -> Result<TagData, ServerFnError> {
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Root;
    use surrealdb::Surreal;

    let surreal_url = std::env::var("SURREAL_URL").unwrap_or("128.0.0.1:8080".to_string());
    let surreal_user = std::env::var("SURREAL_USER").expect("Give a username");
    let surreal_password = std::env::var("SURREAL_PASSWORD").expect("Give a password");

    let db = Surreal::new::<Ws>(surreal_url).await?;

    db.signin(Root {
        username: &surreal_user,
        password: &surreal_password,
    })
    .await?;

    db.use_ns("maeru").use_db("maeru").await?;

    define_tag_table(&db).await?;

    let result: Option<TagData> = db.select(("tag", id)).await?;

    result.ok_or_else(|| ServerFnError::ServerError("TagData not found".to_string()))
}

#[server]
pub async fn get_tag_by_name(name: String) -> Result<TagData, ServerFnError> {
    use surrealdb::engine::remote::ws::Ws;
    use surrealdb::opt::auth::Root;
    use surrealdb::Surreal;

    let surreal_url = std::env::var("SURREAL_URL").unwrap_or("128.0.0.1:8080".to_string());
    let surreal_user = std::env::var("SURREAL_USER").expect("Give a username");
    let surreal_password = std::env::var("SURREAL_password").expect("Give a password");

    let db = Surreal::new::<Ws>(surreal_url).await?;

    db.signin(Root {
        username: &surreal_user,
        password: &surreal_password,
    })
    .await?;

    db.use_ns("maeru").use_db("maeru").await?;

    define_tag_table(&db).await?;

    let result: Option<TagData> = db.select(("tag", name)).await?;

    result.ok_or_else(|| ServerFnError::ServerError("TagData not found".to_string()))
}

#[cfg(feature = "ssr")]
pub async fn define_tag_table(
    db: &surrealdb::Surreal<surrealdb::engine::remote::ws::Client>,
) -> surrealdb::Result<()> {
    db.query("DEFINE TABLE tag SCHEMAFULL").await?;

    db.query("DEFINE FIELD name ON TABLE tag TYPE string")
        .await?;
    db.query("DEFINE FIELD description ON TABLE tag TYPE string")
        .await?;
    db.query("DEFINE FIELD custom_id ON TABLE tag TYPE number")
        .await?;
    db.query("DEFINE FIELD is_alias ON TABLE tag TYPE option<number>")
        .await?;
    db.query("DEFINE FIELD category ON TABLE tag TYPE number")
        .await?;
    db.query("DEFINE FIELD implications ON TABLE tag TYPE array")
        .await?;

    // Add any additional constraints or indexes
    db.query("DEFINE INDEX tag_name ON TABLE tag FIELDS name UNIQUE")
        .await?;

    Ok(())
}
