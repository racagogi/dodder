use std::{collections::HashMap, fs, path::PathBuf};

use dodder::{
    index::{add_node, read_index, write_index},
    node::{root, Status},
};

fn main() {
    init();
    let mut index = read_index();
    println!("{index:?}");
    add_node(
        &mut index,
        "toc".to_string(),
        "norg",
        Some(Status::Recurring),
        None,
        None,
        &PathBuf::new().join("/"),
        true,
        true,
    );
    write_index(&index);
}

fn init() {
    let dodder_path = PathBuf::new().join(".").join(".dodder");
    if !dodder_path.exists() {
        fs::create_dir(&dodder_path).unwrap();
    }
    let data_path = dodder_path.join("data");
    if !data_path.exists() {
        fs::create_dir(&data_path).unwrap();
        let mut index = HashMap::new();
        index.insert(PathBuf::new().join("/"), root());
        write_index(&index);
    }
}
