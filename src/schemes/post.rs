#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Safety {
    Safe,
    Sketchy,
    Unsafe,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PostType {
    Image,
    Video,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Post {
    id: u64,
    image_height: u32,
    image_width: u32,
    mime_type: mime::Mime,
    post_type: PostType,
    safety: Safety,
    sha256_hash: [u8; 32],
    uploader_id: u32,
    tags: Vec<u64>,
}
