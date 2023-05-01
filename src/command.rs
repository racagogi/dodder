use std::{collections::HashMap, fs, path::PathBuf, str::FromStr};

use crate::{
    index::{Index, Tree},
    node::root,
};

pub fn init() {
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

pub fn print(from: &Option<&String>, verbose: bool) -> String {
    let index = Tree::read_index();
    match from {
        Some(k) => index.print(&PathBuf::from_str(k).unwrap(), 0, verbose),
        None => index.print(&PathBuf::from_str("/").unwrap(), 0, verbose),
    }
}

pub fn move_node(from: Option<String>, to: Option<String>, is_sibber: bool, is_last: bool) {
    let mut index = Tree::read_index();
    let from = from.expect("need from");
    let from = PathBuf::from_str(&from).unwrap();
    let to = to.expect("need to");
    let position = crate::index::Position {
        position: PathBuf::from_str(&to).unwrap(),
        is_front: !is_last,
        is_parent: !is_sibber,
    };
    index.move_node(&from, &position);
    index.write_index();
}
