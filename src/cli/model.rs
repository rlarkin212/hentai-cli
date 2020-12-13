use std::path::PathBuf;

pub struct CliModel {
    pub media: Vec<String>,
    pub current_directory: PathBuf,
}