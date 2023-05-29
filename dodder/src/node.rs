use std::fmt::Write;
use std::{collections::HashSet, path::PathBuf};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Node {
    pub ctime: DateTime<Local>,
    pub mtime: DateTime<Local>,
    pub path: PathBuf,
    pub leafs: Vec<PathBuf>,
    pub links: HashSet<PathBuf>,
    pub parent: Option<PathBuf>,
    pub state: State,
    pub name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub gtd: Status,
    pub stime: DateTime<Local>,
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

impl Status {
    fn print(&self) -> &str {
        match self {
            Status::None => "",
            Status::Undone => "",
            Status::Done => "",
            Status::NeedWork => "",
            Status::Urgent => "",
            Status::Recurring => "",
            Status::Pending => "",
            Status::Hold => "",
            Status::Cancelled => "ﰸ",
        }
    }
}

impl Node {
    pub fn add_child(&mut self, i: usize, child: &PathBuf) {
        self.leafs.insert(i, child.to_owned())
    }

    pub fn set_parent(&mut self, parent: &PathBuf) {
        self.parent = Some(parent.to_owned())
    }

    pub fn add_link(&mut self, link: &PathBuf) {
        self.links.insert(link.to_owned());
    }

    pub fn remove_link(&mut self, link: &PathBuf) {
        self.links.remove(link);
    }

    pub fn get_child(&self, leaf: &PathBuf) -> Option<usize> {
        self.leafs.iter().position(|x| x == leaf)
    }

    pub fn remove_child(&mut self, child: &PathBuf) {
        if let Some(i) = self.get_child(child) {
            self.leafs.remove(i);
        }
    }

    pub fn is_root(&self) -> bool {
        if let None = self.parent {
            true
        } else {
            false
        }
    }

    pub fn print(&self, verbose: bool) -> String {
        let mut temp = String::new();
        if verbose {
            writeln!(
                temp,
                "{}",
                format!(
                    "{} {}  {}",
                    self.state.gtd.print(),
                    self.name,
                    self.path.to_str().unwrap()
                )
            )
        } else {
            writeln!(
                temp,
                "{}",
                format!("{} {}", self.state.gtd.print(), self.name,)
            )
        }
        .unwrap();
        temp
    }
}

pub fn root() -> Node {
    Node {
        ctime: Local::now(),
        mtime: Local::now(),
        path: PathBuf::new().join("/"),
        leafs: Vec::new(),
        links: HashSet::new(),
        parent: None,
        state: State {
            gtd: Status::None,
            stime: Local::now(),
        },
        name: "root".to_string(),
    }
}

pub fn root_path() -> PathBuf {
    PathBuf::new().join("/")
}
