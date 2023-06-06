use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::config::{Config, CGTD};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Leaf {
    leafdata: LeafData,
    visible: bool,
    childs: Vec<Index>,
    links: HashSet<Index>,
    index: Index,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct LeafData {
    message: String,
    gtd: GTD,
    path: PathBuf,
    ctime: DateTime<Utc>,
    mtime: DateTime<Utc>,
    stime: DateTime<Utc>,
}

pub type Index = usize;
#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub enum GTD {
    None,
    Undone,
    Done,
    NeedWork,
    Urgent,
    Recurring,
    Pending,
    Hold,
    Cancelled,
}

impl GTD {
    pub fn to_symbol(&self, symbol_list: CGTD) -> char {
        match self {
            GTD::None => match symbol_list.none {
                Some(c) => c,
                None => '󰄱',
            },
            GTD::Undone => match symbol_list.undone {
                Some(c) => c,
                None => '󰄮',
            },
            GTD::Done => match symbol_list.done {
                Some(c) => c,
                None => '󰄲',
            },
            GTD::NeedWork => match symbol_list.need_work {
                Some(c) => c,
                None => '󰱒',
            },
            GTD::Urgent => match symbol_list.urgent {
                Some(c) => c,
                None => '⚠',
            },
            GTD::Recurring => match symbol_list.recurring {
                Some(c) => c,
                None => '󰑖',
            },
            GTD::Pending => match symbol_list.pending {
                Some(c) => c,
                None => '󰏤',
            },
            GTD::Hold => match symbol_list.hold {
                Some(c) => c,
                None => '󰐊',
            },
            GTD::Cancelled => match symbol_list.cancelled {
                Some(c) => c,
                None => '󰜺',
            },
        }
    }
}

impl Leaf {
    pub fn new(leafdata: LeafData, index: Index) -> Leaf {
        Leaf {
            leafdata,
            visible: true,
            childs: Vec::new(),
            links: HashSet::new(),
            index,
        }
    }
    pub fn get_index(&self) -> Index {
        self.index
    }

    pub fn get_childs(&self) -> Vec<Index> {
        self.childs.to_owned()
    }

    pub fn get_links(&self) -> HashSet<Index> {
        self.links.to_owned()
    }

    pub fn get_visible(&self) -> bool {
        self.visible
    }

    pub fn add_child(&mut self, is_first: bool, index: Index) {
        if is_first {
            self.childs.insert(0, index);
        } else {
            self.childs.push(index);
        }
    }

    pub fn remove_child(&mut self, index: Index) {
        if let Some(i) = self.childs.iter().position(|&x| x == index) {
            self.childs.remove(i);
        }
    }

    pub fn add_link(&mut self, index: Index) {
        self.links.insert(index);
    }

    pub fn remove_link(&mut self, index: Index) {
        self.links.remove(&index);
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
    }

    pub fn print(&self, config: &Config) -> String {
        self.leafdata.print(config.get_symbols())
    }
}

impl LeafData {
    pub fn new(
        message: &str,
        path: Option<PathBuf>,
        gtd: GTD,
        stime: DateTime<Utc>,
        is_global: bool,
        config: &Config,
    ) -> LeafData {
        match path {
            Some(p) => LeafData {
                message: message.to_string(),
                gtd,
                path: p,
                ctime: Utc::now(),
                mtime: Utc::now(),
                stime,
            },
            None => LeafData {
                message: message.to_string(),
                gtd,
                path: make_file(message, is_global, config),
                ctime: Utc::now(),
                mtime: Utc::now(),
                stime,
            },
        }
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    pub fn set_status(&mut self, gtd: GTD, stime: DateTime<Utc>) {
        self.gtd = gtd;
        self.stime = stime;
    }

    pub fn update_time(&mut self) {
        let file = fs::File::open(&self.path).unwrap();
        let meta = file.metadata().unwrap();
        self.ctime = meta.created().unwrap().into();
        self.mtime = meta.modified().unwrap().into();
    }

    pub fn print(&self, symbol_list: CGTD) -> String {
        let mut temp = String::new();
        write!(temp, "{} {}", self.gtd.to_symbol(symbol_list), self.message).unwrap();
        temp
    }
}

fn make_file(message: &str, is_global: bool, config: &Config) -> PathBuf {
    let extension = config.get_extension();
    let file_name = format!(
        "{}{}.{}",
        Utc::now().timestamp().to_string(),
        message,
        extension
    );
    let path = if is_global {
        PathBuf::new().join(config.get_global_path()).join("data")
    } else {
        PathBuf::new().join(".").join(".dodder").join("data")
    };
    fs::create_dir_all(&path).unwrap();
    let file_path = path.join(file_name);
    fs::File::create(&file_path).expect("can nopt make file");
    file_path
}
