#[derive(Clone, PartialEq, Debug)]
pub struct Tag {
    name: String,
    description: String,
    id: u64,
    is_alias: Alias,
    category: u8,
    implications: Vec<u64>,
}

impl Tag {
    pub fn new(
        name: String,
        description: String,
        id: u64,
        is_alias: Alias,
        category: u8,
        implications: Vec<u64>,
    ) -> Tag {
        Tag {
            name,
            description,
            id,
            is_alias,
            category,
            implications,
        }
    }
}

impl Default for Tag {
    fn default() -> Tag {
        Tag {
            name: String::from("newtag"),
            description: String::new(),
            id: 0,
            is_alias: Alias::No,
            category: 1,
            implications: vec![],
        }
    }
}

/// No is not an alias and in yes u64 is actual tag id
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Alias {
    No,
    Yes(u64),
}
