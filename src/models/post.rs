use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Safety {
    Safe,
    Sketchy,
    Unsafe,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum PostType {
    Image,
    Video,
}

/// This is the struct for posts
#[serde_as]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Post {
    custom_id: u64,
    image_height: u32,
    image_width: u32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    mime: mime::Mime,
    post_type: PostType,
    safety: Safety,
    sha256_hash: [u8; 32],
    md5_hash: [u8; 16],
    /// link to the source
    source: String,
    uploaded: DateTime<Utc>,
    uploader_id: u32,
    tags: Vec<u64>,
    relations: Vec<u64>,
}

#[derive(thiserror::Error, Debug)]
pub enum PostCreateError {
    /// First Vec<64> is post ids second Vec<f32> is simmilarity
    #[error("Found simmilar/dublicate posts: {0:?}")]
    DublicatePostsFound(Vec<u64>,Vec<f32>),

    #[error("File type should be image or video")]
     InvalidFileType(),
}


#[derive(thiserror::Error, Debug)]
pub enum PostSearchError {
    #[error("Found no posts with these tags")]
    NoSuchPosts(),

    #[error("Your query is wrong")]
    InvalidQuery(),
}
