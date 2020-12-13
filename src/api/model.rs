use bytes::Bytes;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiModel {
    pub id: i32,
    pub media_id: String,
    pub title: Title,
    pub images: Images,
    pub num_pages: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Title {
    pub english: String,
    pub japanese: String,
    pub pretty: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Images {
    pub pages: Vec<Pages>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Pages {
    pub t: String,
    pub w: i32,
    pub h: i32,
}
pub struct ImageModel {
    pub file_name: String,
    pub file_contents: Bytes,
}
