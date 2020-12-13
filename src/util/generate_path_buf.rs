use std::path::PathBuf;

pub fn generate_path_buf(name: &String) -> PathBuf {
    let current = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let new_path_buf = PathBuf::from(format!("{}/{}", current, name));

    return new_path_buf;
}
