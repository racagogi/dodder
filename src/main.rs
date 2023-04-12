use std::{collections::HashMap, fs, path::PathBuf};

use dodder::{
    index::{add_link, add_node, move_node, read_index, remove_node, write_index},
    node::{root, Status},
};

fn main() {
    init();
    let mut index = read_index();
    add_node(
        &mut index,
        None,
        &"toc".to_string(),
        "norg",
        Some(Status::Recurring),
        None,
        &PathBuf::new().join("/"),
        true,
        true,
    );
    add_node(
        &mut index,
        Some(&PathBuf::new().join(".").join("src").join("main.rs")),
        &"main".to_string(),
        "md",
        None,
        None,
        &PathBuf::new().join("/"),
        false,
        true,
    );
    add_node(
        &mut index,
        Some(&PathBuf::new().join(".").join("src").join("lib.rs")),
        &"index".to_string(),
        "py",
        None,
        None,
        &PathBuf::new().join("/"),
        false,
        true,
    );
    add_node(
        &mut index,
        Some(&PathBuf::new().join(".").join("src").join("index.rs")),
        &"index".to_string(),
        "md",
        None,
        None,
        &PathBuf::new().join(".").join("src").join("lib.rs"),
        false,
        true,
    );
    add_node(
        &mut index,
        Some(&PathBuf::new().join(".").join("src").join("node.rs")),
        &"node".to_string(),
        "md",
        None,
        None,
        &PathBuf::new().join(".").join("src").join("index.rs"),
        false,
        true,
    );
    add_link(
        &mut index,
        &PathBuf::new().join(".").join("src").join("index.rs"),
        &PathBuf::new().join(".").join("src").join("lib.rs"),
    );
    add_link(
        &mut index,
        &PathBuf::new().join(".").join("src").join("index.rs"),
        &PathBuf::new().join(".").join("src").join("lib.rs"),
    );
    move_node(
        &mut index,
        &PathBuf::new().join(".").join("src").join("node.rs"),
        &PathBuf::new().join(".").join("src").join("lib.rs"),
        true,
        false,
    );
    remove_node(
        &mut index,
        &PathBuf::new().join(".").join("src").join("index.rs"),
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
