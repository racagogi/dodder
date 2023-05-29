use std::fmt::Write;
use std::io;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufReader, Read},
    path::PathBuf,
    time::SystemTime,
};

use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};

use crate::node::{Node, State, Status};

pub type Tree = HashMap<PathBuf, Node>;

pub enum FilePath {
    Path(PathBuf),
    Extension(String),
}

pub struct Position {
    pub position: PathBuf,
    pub is_front: bool,
    pub is_parent: bool,
}

pub trait Index {
    fn read_index() -> Tree;
    fn write_index(&self);
    fn add_node(&mut self, name: &str, state: Option<State>, path: FilePath, position: &Position);
    fn remove_node(&mut self, node: &PathBuf);
    fn move_node(&mut self, key: &PathBuf, position: &Position);
    fn add_link(&mut self, a: &PathBuf, b: &PathBuf);
    fn remove_link(&mut self, a: &PathBuf, b: &PathBuf);
    fn print(&self, key: &PathBuf, depth: usize, verbose: bool) -> String;
}

impl Index for Tree {
    fn read_index() -> Tree {
        let index_path = PathBuf::new().join(".").join(".dodder").join("index.json");
        let file = fs::File::open(index_path).expect("can not read");
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader).expect("can not parse");
        data
    }

    fn write_index(&self) {
        let index_path = PathBuf::new().join(".").join(".dodder").join("index.json");
        let mut file = fs::OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(true)
            .open(index_path)
            .expect("can not open file");
        let mut reader = BufReader::new(&file);
        let mut backup = String::new();
        reader.read_to_string(&mut backup).expect("can not read");
        let hash_json = match serde_json::to_vec_pretty(self) {
            Ok(v) => v,
            Err(_) => backup.as_bytes().to_owned(),
        };
        io::Write::write(&mut file, &hash_json).expect("can not write");
        io::Write::flush(&mut file).unwrap();
    }

    fn add_node(&mut self, name: &str, state: Option<State>, path: FilePath, position: &Position) {
        let path = ensure_path(path, name);
        let mut node = make_node(name, &path, state);
        if position.is_parent {
            add_node_parent(
                self,
                &path,
                &mut node,
                &position.position,
                position.is_front,
            );
        } else {
            add_node_sibber(
                self,
                &path,
                &mut node,
                &position.position,
                position.is_front,
            );
        }
    }

    fn remove_node(&mut self, node: &PathBuf) {
        if let Some(parent) = self.remove(node) {
            for c in parent.leafs.iter() {
                self.remove_node(c)
            }
        }
        self.iter_mut().for_each(|(_, v)| {
            v.remove_child(node);
            v.links.remove(node);
        });
    }

    fn move_node(&mut self, key: &PathBuf, position: &Position) {
        let mut node = self.get(key).expect("node not eixst").to_owned();
        if let Some(v) = self.get_mut(node.parent.as_ref().unwrap()) {
            v.remove_child(key);
        }
        if position.is_parent {
            add_node_parent(self, key, &mut node, &position.position, position.is_front);
        } else {
            add_node_sibber(self, key, &mut node, &position.position, position.is_front);
        }
    }

    fn add_link(&mut self, a: &PathBuf, b: &PathBuf) {
        let a_node = self.get(a).unwrap();
        let b_node = self.get(b).unwrap();
        let mut a_node = a_node.clone();
        a_node.add_link(b);
        let mut b_node = b_node.clone();
        b_node.add_link(a);
        self.insert(a.to_owned(), a_node);
        self.insert(b.to_owned(), b_node);
    }

    fn remove_link(&mut self, a: &PathBuf, b: &PathBuf) {
        let a_node = self.get(a).unwrap();
        let b_node = self.get(a).unwrap();
        let mut a_node = a_node.clone();
        let mut b_node = b_node.clone();
        a_node.remove_link(b);
        b_node.remove_link(a);
        self.insert(a.to_owned(), a_node);
        self.insert(b.to_owned(), b_node);
    }

    fn print(&self, key: &PathBuf, depth: usize, verbose: bool) -> String {
        let node = self.get(key).unwrap();
        let mut temp = String::new();
        write!(
            temp,
            "{}{}",
            String::from("  ").repeat(depth),
            node.print(verbose)
        )
        .unwrap();
        for i in node.leafs.iter() {
            write!(temp, "{}", self.print(i, depth + 1, verbose)).unwrap();
        }
        temp
    }
}

fn sys_to_chro(stime: &SystemTime) -> DateTime<Local> {
    let secs = 1 + stime
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    DateTime::<Local>::from_utc(
        NaiveDateTime::from_timestamp_opt(secs as i64, 0).unwrap(),
        Local.timestamp_opt(0, 0).unwrap().offset().fix(),
    )
}

fn ctime(path: &PathBuf) -> DateTime<Local> {
    let file = fs::File::open(path).expect("can not open file");
    let ctime = file.metadata().unwrap().created().unwrap();
    sys_to_chro(&ctime)
}

fn mtime(path: &PathBuf) -> DateTime<Local> {
    let file = fs::File::open(path).expect("can not open file");
    let ctime = file.metadata().unwrap().modified().unwrap();
    sys_to_chro(&ctime)
}

fn make_file(name: &str, extension: &str) -> PathBuf {
    let file_name = format!(
        "{}{}.{}",
        Utc::now().timestamp().to_string(),
        name,
        extension
    );
    let path = PathBuf::new()
        .join(".")
        .join(".dodder")
        .join("data")
        .join(file_name);
    fs::File::create(&path).expect("can not make file");
    path
}

fn ensure_path(path: FilePath, name: &str) -> PathBuf {
    match path {
        FilePath::Path(p) => p,
        FilePath::Extension(e) => make_file(name, &e),
    }
}

fn make_node(name: &str, path: &PathBuf, state: Option<State>) -> Node {
    Node {
        ctime: ctime(path),
        mtime: mtime(path),
        path: path.to_owned(),
        leafs: Vec::new(),
        links: HashSet::new(),
        parent: None,
        state: match state {
            Some(s) => State {
                gtd: s.gtd,
                stime: s.stime,
            },
            None => State {
                gtd: Status::None,
                stime: Local::now(),
            },
        },
        name: name.to_owned(),
    }
}

fn add_node_sibber(
    index: &mut Tree,
    key: &PathBuf,
    node: &mut Node,
    sibber: &PathBuf,
    is_front: bool,
) {
    let sibber_node = index.get(sibber).unwrap();
    let parent = sibber_node.parent.clone().unwrap();
    let mut parent_node = index.get(&parent).unwrap().to_owned();
    if let None = parent_node.get_child(key) {
        let i = parent_node.get_child(sibber).unwrap();
        if is_front {
            parent_node.add_child(i, key);
        } else {
            parent_node.add_child(i + 1, key);
        }
    }
    node.set_parent(&parent);
    node.set_parent(&parent);
    index.insert(key.to_owned(), node.to_owned());
    index.insert(parent.to_owned(), parent_node);
}

fn add_node_parent(
    index: &mut Tree,
    key: &PathBuf,
    node: &mut Node,
    parent: &PathBuf,
    is_front: bool,
) {
    let mut parent_node = index.get(parent).expect("not eixst parent").to_owned();
    if let None = parent_node.get_child(key) {
        if is_front {
            parent_node.add_child(0, key);
        } else {
            parent_node.add_child(parent_node.leafs.len(), key);
        }
    }
    node.set_parent(parent);
    node.set_parent(parent);
    index.insert(key.to_owned(), node.to_owned());
    index.insert(parent.to_owned(), parent_node);
}
