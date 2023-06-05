use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    leaf::{Index, Leaf, LeafData, GTD},
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dodder {
    tree: HashMap<Index, Leaf>,
    index: Vec<Index>,
    max_index: Index,
}

impl Dodder {
    pub fn new(is_global: bool, config: &Config) -> Dodder {
        let root_data = LeafData::new("root", None, GTD::None, Utc::now(), is_global, config);
        let root = Leaf::new(root_data, 0);
        let mut tree = HashMap::<Index, Leaf>::new();
        tree.insert(0, root);
        Dodder {
            tree,
            index: vec![0],
            max_index: 0,
        }
    }

    pub fn write(&self, is_global: bool) {
        let temp = serde_json::to_vec_pretty(self).unwrap();
        let config = Config::read();
        let path = if is_global {
            PathBuf::new().join(config.get_global_path())
        } else {
            PathBuf::new().join(".").join(".dodder")
        };
        let file_path = path.join("index.json");
        if !path.exists() {
            fs::create_dir_all(path).unwrap();
        }
        if !file_path.exists() {
            let mut file = fs::File::create(file_path).unwrap();
            file.write_all(&temp).unwrap();
        } else {
            let mut file = fs::File::options()
                .truncate(true)
                .write(true)
                .open(file_path)
                .unwrap();
            file.write_all(&temp).unwrap();
        }
    }

    pub fn read(is_global: bool) -> Dodder {
        let config = Config::read();
        let path = if is_global {
            PathBuf::new()
                .join(config.get_global_path())
                .join("index.json")
        } else {
            PathBuf::new().join(".").join(".dodder").join("index.json")
        };
        if !path.exists() {
            let init = Dodder::new(is_global, &config);
            init.write(is_global);
            init
        } else {
            let mut file = fs::File::open(path).unwrap();
            let mut temp = String::new();
            file.read_to_string(&mut temp).unwrap();
            let data = serde_json::from_str(&temp).unwrap();
            data
        }
    }

    fn dfs(&self, index: Index) -> Vec<Index> {
        let leaf = self.tree.get(&index).unwrap();
        let childs = leaf.get_childs();
        let mut temp = Vec::new();
        temp.push(index);
        if childs.is_empty() {
            temp
        } else {
            for i in childs {
                temp.append(&mut self.dfs(i));
            }
            temp
        }
    }

    fn get_leaf(&self, index: Index) -> Leaf {
        self.tree.get(&index).unwrap().to_owned()
    }

    pub fn add_child_first(&mut self, leafdata: LeafData, parent: Index) {
        let mut parent_leaf = self.get_leaf(parent);
        let index = self.max_index + 1;
        self.max_index += 1;
        parent_leaf.add_child(true, index);
        let leaf = Leaf::new(leafdata, index);
        self.tree.insert(index, leaf);
        self.tree.insert(parent, parent_leaf);
        self.index = self.dfs(0);
    }

    pub fn add_child_last(&mut self, leafdata: LeafData, parent: Index) {
        let mut parent_leaf = self.get_leaf(parent);
        let index = self.max_index + 1;
        self.max_index += 1;
        parent_leaf.add_child(false, index);
        let leaf = Leaf::new(leafdata, index);
        self.tree.insert(index, leaf);
        self.tree.insert(parent, parent_leaf);
        self.index = self.dfs(0);
    }

    pub fn add_link(&mut self, from: Index, to: Index) {
        let mut from_leaf = self.get_leaf(from);
        let mut to_leaf = self.get_leaf(to);
        from_leaf.add_link(to);
        to_leaf.add_link(from);
        self.tree.insert(from, from_leaf);
        self.tree.insert(to, to_leaf);
    }

    pub fn remove_link(&mut self, from: Index, to: Index) {
        let mut from_leaf = self.get_leaf(from);
        let mut to_leaf = self.get_leaf(to);
        from_leaf.remove_link(to);
        to_leaf.remove_link(from);
        self.tree.insert(from, from_leaf);
        self.tree.insert(to, to_leaf);
    }

    pub fn remove_child(&mut self, parent: Index, child: Index) {
        let mut parent_leaf = self.get_leaf(parent);
        parent_leaf.remove_child(child);
        self.tree.insert(parent, parent_leaf);
        self.remove_leaf(child);
        self.index = self.dfs(0);
    }

    fn remove_leaf(&mut self, index: Index) {
        let leaf = self.get_leaf(index);
        let leaf_childs = leaf.get_childs();
        let leaf_links = leaf.get_links();
        self.tree.remove(&index);
        for i in leaf_links {
            self.remove_link(index, i);
        }
        for i in leaf_childs {
            self.remove_leaf(i);
        }
    }
}
