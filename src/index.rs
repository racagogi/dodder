use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufReader, Read, Write},
    path::PathBuf,
    time::SystemTime,
};

use chrono::{DateTime, NaiveDateTime, Utc};

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
    name: String,
    extension: &str,
    status: Option<Status>,
    stime: Option<DateTime<Utc>>,
    path: Option<&PathBuf>,
    from_node: &PathBuf,
    is_parent: bool,
    is_front: bool,
) {
    let parent_path = if is_parent {
        index.get(from_node).expect("not exist parent");
        from_node.to_owned()
    } else {
        index
            .get(from_node)
            .unwrap()
            .parent
            .clone()
            .expect("can not add root siddering")
    };
    let node_path = match path {
        Some(p) => p.to_owned(),
        None => make_file(&name, extension),
    };

    if let Some(_) = index.get(&node_path) {
        eprint!("already exist node");
    }

    let mut node = make_node(name, &node_path, status, stime);
    node.set_parent(&parent_path);
    let parent = index.get(&parent_path).unwrap();
    let i = if is_parent {
        if is_front {
            0
        } else {
            parent.child.len()
        }
    } else {
        let sidder_i = parent.get_child(from_node).unwrap();
        if is_front {
            sidder_i
        } else {
            sidder_i + 1
        }
    };
    let mut new_parent = parent.clone();
    new_parent.add_child(i, &node_path);
    index.insert(node_path, node);
    index.insert(parent_path, new_parent);
}

pub fn remove_node(index: &mut Index, node: &PathBuf) {
    if let Some(parent) = index.remove(node) {
        for c in parent.child.iter() {
            remove_node(index, c)
        }
    }
    index.iter_mut().for_each(|(_, v)| v.remove_child(node));
}

pub fn move_node(
    index: &mut Index,
    node: &PathBuf,
    old: &PathBuf,
    new: &PathBuf,
    is_front: bool,
    is_parent: bool,
) {
    if let Some(old_node) = index.get(old) {
        if let Some(new_node) = index.get(new) {
            if let Some(node_value) = index.get(node) {
                todo!()
            }
        }
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

fn sys_to_chro(stime: &SystemTime) -> DateTime<Utc> {
    let secs = 1 + stime
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(secs as i64, 0).unwrap(),
        Utc,
    )
}

fn ctime(path: &PathBuf) -> DateTime<Utc> {
    let file = fs::File::open(path).expect("can not open file");
    let ctime = file.metadata().unwrap().created().unwrap();
    sys_to_chro(&ctime)
}

fn mtime(path: &PathBuf) -> DateTime<Utc> {
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

fn make_node(
    name: String,
    path: &PathBuf,
    status: Option<Status>,
    stime: Option<DateTime<Utc>>,
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
        name,
    }
}
