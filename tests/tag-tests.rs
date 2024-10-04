#[cfg(feature = "ssr")]
pub mod server_only {
    use maerbooru::server_only::tag::add_new_tag;
    use surrealdb::engine::local::Mem;

    use maerbooru::server_only::tag::get_tag_by_name;
    use maerbooru::server_only::tag::get_tag_by_id;
    use maerbooru::models::tag::Tag;
    use maerbooru::server_only::tag::define_tag_table;

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
            use_count: 0,
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
            use_count: 0,
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

    #[allow(clippy::needless_return)]
    #[tokio::test]
    async fn list_tags_by_page() {todo!();} // TODO: add proper testing for paginating.
    
    #[allow(clippy::needless_return)]
    #[tokio::test]
    async fn tag_query_generation() {todo!();} // TODO: add proper testing search query generation
}