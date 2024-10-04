use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub custom_id: u64,
    pub name: String,
    pub description: String,
    pub is_alias: Option<u64>,
    pub category: u8,
    pub implications: Vec<u64>,
    pub use_count: u64,
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
            use_count: 0,
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
            use_count: 0,
        }
    }
}
