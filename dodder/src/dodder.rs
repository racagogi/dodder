use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{config::Config, leaf::Leaf};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dodder {
    leaf: Leaf,
    leafs: Vec<Dodder>,
}

impl Dodder {
    pub fn new(is_global: bool, config: &Config) -> Dodder {
        Dodder {
            leaf: Leaf::new(
                "root",
                // Some(PathBuf::from("/")),
                None,
                crate::leaf::GTD::None,
                Utc::now(),
                is_global,
                config,
            ),
            leafs: Vec::new(),
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
}
