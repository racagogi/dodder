use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufReader, Read},
    path::PathBuf,
    time::SystemTime,
};

use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};

use crate::node::{Node, Status};

pub type Index = HashMap<PathBuf, Node>;

pub fn read_index() -> Index {
    let index_path = PathBuf::new().join(".").join(".dodder").join("index.json");
    let file = fs::File::open(index_path).expect("can not read");
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).expect("can not parse");
    data
}

pub fn write_index(index: &Index) {
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
    let hash_json = match serde_json::to_vec_pretty(index) {
        Ok(v) => v,
        Err(_) => backup.as_bytes().to_owned(),
    };
    file.write(&hash_json).expect("can not write");
    file.flush().unwrap();
}

pub fn add_node(
    index: &mut Index,
    path: Option<&PathBuf>,
    name: &String,
    extension: &str,
    status: Option<Status>,
    stime: Option<DateTime<Local>>,
    position: &PathBuf,
    is_front: bool,
    is_parent: bool,
) {
    let path = ensure_path(path, name, extension);
    let mut node = make_node(name, &path, status, stime);
    if is_parent {
        add_node_parent(index, &path, &mut node, position, is_front);
    } else {
        add_node_sibber(index, &path, &mut node, position, is_front);
    }
}

pub fn remove_node(index: &mut Index, node: &PathBuf) {
    if let Some(parent) = index.remove(node) {
        for c in parent.child.iter() {
            remove_node(index, c)
        }
    }
    index.iter_mut().for_each(|(_, v)| {
        v.remove_child(node);
        v.links.remove(node);
    });
}

pub fn move_node(
    index: &mut Index,
    key: &PathBuf,
    position: &PathBuf,
    is_front: bool,
    is_parent: bool,
) {
    let mut node = index.get(key).expect("node not eixst").to_owned();
    if let Some(v) = index.get_mut(node.parent.as_ref().unwrap()) {
        v.remove_child(key);
    }
    if is_parent {
        add_node_parent(index, key, &mut node, position, is_front);
    } else {
        add_node_sibber(index, key, &mut node, position, is_front);
    }
}

pub fn add_link(index: &mut Index, a: &PathBuf, b: &PathBuf) {
    let a_node = index.get(a).unwrap();
    let b_node = index.get(b).unwrap();
    let mut a_node = a_node.clone();
    a_node.add_link(b);
    let mut b_node = b_node.clone();
    b_node.add_link(a);
    index.insert(a.to_owned(), a_node);
    index.insert(b.to_owned(), b_node);
}

pub mod print {
    use super::Index;
    use std::{fmt::Write, path::PathBuf};
    pub fn print_tree(index: &Index, key: &PathBuf, depth: usize) -> String {
        let node = index.get(key).unwrap();
        let mut temp = String::new();
        write!(temp, "{}{}", String::from("\t").repeat(depth), node.print()).unwrap();
        for i in node.child.iter() {
            write!(temp, "{}", print_tree(index, i, depth + 1)).unwrap();
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

fn ensure_path(path: Option<&PathBuf>, name: &String, extension: &str) -> PathBuf {
    match path {
        Some(p) => p.to_owned(),
        None => make_file(name, extension),
    }
}

fn make_node(
    name: &String,
    path: &PathBuf,
    status: Option<Status>,
    stime: Option<DateTime<Local>>,
) -> Node {
    Node {
        ctime: ctime(path),
        mtime: mtime(path),
        stime: match stime {
            Some(t) => t,
            None => mtime(path),
        },
        path: path.to_owned(),
        child: Vec::new(),
        links: HashSet::new(),
        parent: None,
        status: match status {
            Some(s) => s,
            None => Status::None,
        },
        name: name.to_owned(),
    }
}

fn add_node_sibber(
    index: &mut Index,
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
    index: &mut Index,
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
            parent_node.add_child(parent_node.child.len(), key);
        }
    }
    node.set_parent(parent);
    node.set_parent(parent);
    index.insert(key.to_owned(), node.to_owned());
    index.insert(parent.to_owned(), parent_node);
}
