use std::{collections::HashMap, fs, path::PathBuf};

use dodder::{
    index::{FilePath::{Extension, Path}, Index, Position, Tree},
    node::{root, root_path},
};

fn main() {
    init();
    let mut index = Tree::read_index();
    index.add_node(
        "test",
        None,
        Extension("norg".to_string()),
        &Position {
            position: root_path(),
            is_front: true,
            is_parent: true,
        },
    );
    index.add_node(
        "test",
        None,
        Path(PathBuf::from("./src/main.rs")),
        &Position {
            position: root_path(),
            is_front: true,
            is_parent: true,
        },
    );
    index.add_node(
        "to do",
        None,
        Extension("norg".to_string()),
        &Position {
            position: root_path(),
            is_front: true,
            is_parent: true,
        },
    );
    index.add_node(
        "undo",
        None,
        Extension("norg".to_string()),
        &Position {
            position: root_path(),
            is_front: true,
            is_parent: true,
        },
    );
    index.remove_node(&PathBuf::from("./src/main.rs"));
    index.write_index();
    println!("{}",index.print(&root_path(), 0, true));
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
        index.write_index();
    }
}
