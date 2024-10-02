use leptos::{ServerFnError, ServerFnErrorErr};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::{
    engine::remote::ws::{self, Client},
    opt::auth::Root,
    Surreal,
};

pub async fn get_db_connection() -> Result<Surreal<Client>, ServerFnErrorErr> {
    let surreal_url = std::env::var("SURREAL_URL").unwrap_or("127.0.0.1:8000".to_string());
    let surreal_user = std::env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let surreal_password = std::env::var("SURREAL_PASS").expect("SURREAL_PASS must be set");

    let db = Surreal::new::<Ws>(surreal_url)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    db.signin(Root {
        username: &surreal_user,
        password: &surreal_password,
    })
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    db.use_ns("maeru")
        .use_db("maeru")
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(db)
}
