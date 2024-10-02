use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub custom_id: u64,
    pub name: String,
    pub description: String,
    pub is_alias: Option<u64>,
    pub category: u8,
    pub implications: Vec<u64>,
}

impl Tag {
    pub fn new(
        name: String,
        description: String,
        is_alias: Option<u64>,
        category: u8,
        implications: Vec<u64>,
    ) -> Tag {
        Tag {
            custom_id: 0,
            name,
            description,
            is_alias,
            category,
            implications,
        }
    }
}

impl Default for Tag {
    fn default() -> Tag {
        Tag {
            custom_id: 0,
            name: String::from("newtag"),
            description: String::new(),
            is_alias: None,
            category: 1,
            implications: vec![],
        }
    }
}

#[server(AddNewTag, "/api")]
pub async fn add_new_tag(name: String) -> Result<u64, ServerFnError> {
    let db = crate::db::get_db_connection().await?;

    let new_tag = Tag {
        custom_id: 0, // This will be replaced by the database
        name,
        description: String::new(),
        is_alias: None,
        category: 0,
        implications: vec![],
    };

    // Use the add_new_tag function we created earlier
    match crate::schemes::tag::server_only::add_new_tag(&db, &new_tag).await {
        Ok(custom_id) => Ok(custom_id),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[cfg(feature = "ssr")]
pub mod server_only {
    use anyhow::{anyhow, Ok};
    use regex::Regex;
    use surrealdb::{sql::parse, Connection, Surreal};

    use crate::schemes::tag::Tag;

    fn is_snake_case(s: &str) -> bool {
        let re = Regex::new(r"^[a-z0-9():'_-]+([a-z0-9{}:'_-]+)*$").unwrap();
        re.is_match(s)
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

    async fn define_tag_table<T: Connection>(db: &surrealdb::Surreal<T>) -> anyhow::Result<()> {
        let schema = r#"
        DEFINE TABLE tag SCHEMAFULL;
        
        DEFINE FIELD custom_id ON TABLE tag TYPE number;
        DEFINE FIELD name ON TABLE tag TYPE string;
        DEFINE FIELD description ON TABLE tag TYPE string;
        DEFINE FIELD is_alias ON TABLE tag TYPE option<number>;
        DEFINE FIELD category ON TABLE tag TYPE number;
        DEFINE FIELD implications ON TABLE tag TYPE array;
        
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

    #[cfg(test)]
    mod tests {
        use surrealdb::engine::local::Mem;

        use super::*;

        #[allow(clippy::needless_return)]
        #[tokio::test]
        async fn create_and_find_tag_by_id() {
            let db = surrealdb::Surreal::new::<Mem>(()).await.unwrap();
            db.use_ns("test").use_db("test").await.unwrap();

            define_tag_table(&db).await.unwrap();

            let tag = Tag {
                custom_id: 0,
                name: String::from("test_tag"),
                description: String::from("what the fuck??"),
                is_alias: None,
                category: 0,
                implications: vec![],
            };

            let new_tag_id = add_new_tag(&db, &tag).await.unwrap();

            let found_tag = get_tag_by_id(&db, new_tag_id)
                .await
                .unwrap()
                .expect("tag should exist by now");

            assert_eq!(
                Tag {
                    custom_id: found_tag.custom_id,
                    ..tag
                },
                found_tag
            );
        }

        #[allow(clippy::needless_return)]
        #[tokio::test]
        async fn create_and_find_tag_by_name() {
            let db = surrealdb::Surreal::new::<Mem>(()).await.unwrap();
            db.use_ns("test").use_db("test").await.unwrap();

            define_tag_table(&db).await.unwrap();

            let tag = Tag {
                custom_id: 0,
                name: String::from("test_tag"),
                description: String::from("what the fuck??"),
                is_alias: None,
                category: 0,
                implications: vec![],
            };

            let _ = add_new_tag(&db, &tag).await.unwrap();

            let found_tag = get_tag_by_name(&db, "test_tag".into())
                .await
                .unwrap()
                .expect("tag should exist by now");

            assert_eq!(
                Tag {
                    custom_id: found_tag.custom_id,
                    ..tag
                },
                found_tag
            );
        }
    }
}
