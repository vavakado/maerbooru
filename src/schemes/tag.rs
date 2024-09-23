use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TagData {
    pub name: String,
    pub description: String,
    pub is_alias: Option<u64>,
    pub category: u8,
    pub implications: Vec<u64>,
}

impl TagData {
    pub fn new(
        name: String,
        description: String,
        is_alias: Option<u64>,
        category: u8,
        implications: Vec<u64>,
    ) -> TagData {
        TagData {
            name,
            description,
            is_alias,
            category,
            implications,
        }
    }
}

impl Default for TagData {
    fn default() -> TagData {
        TagData {
            name: String::from("newtag"),
            description: String::new(),
            is_alias: None,
            category: 1,
            implications: vec![],
        }
    }
}

#[cfg(feature = "ssr")]
pub mod server_only {
    use crate::schemes::tag::TagData;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Tag {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub id: Option<surrealdb::sql::Thing>,
        #[serde(flatten)]
        pub data: TagData,
    }
}
