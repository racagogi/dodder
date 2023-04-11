use std::{collections::HashSet, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub ctime: DateTime<Utc>,
    pub mtime: DateTime<Utc>,
    pub stime: DateTime<Utc>,
    pub path: PathBuf,
    pub child: Vec<PathBuf>,
    pub links: HashSet<PathBuf>,
    pub parent: Option<PathBuf>,
    pub status: Status,
    pub name: String,
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub enum Status {
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

impl Node {
    pub fn add_child(&mut self, i: usize, child: &PathBuf) {
        self.child.insert(i, child.to_owned())
    }

    pub fn set_parent(&mut self, parent: &PathBuf) {
        self.parent = Some(parent.to_owned())
    }

    pub fn add_link(&mut self, link: &PathBuf) {
        self.links.insert(link.to_owned());
    }

    pub fn get_child(&self, child: &PathBuf) -> Option<usize> {
        self.child.iter().position(|x| x == child)
    }

    pub fn remove_child(&mut self, child: &PathBuf) {
        if let Some(i) = self.get_child(child) {
            self.child.remove(i);
        }
    }
}

pub fn root() -> Node {
    Node {
        ctime: Utc::now(),
        mtime: Utc::now(),
        stime: Utc::now(),
        path: PathBuf::new().join("/"),
        child: Vec::new(),
        links: HashSet::new(),
        parent: None,
        status: Status::None,
        name: "root".to_string(),
    }
}