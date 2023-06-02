use std::fs;
use std::path::PathBuf;
use std::{fmt::Write, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    global: Option<PathBuf>,
    symbol: Option<CGTD>,
    extension: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CGTD {
    pub none: Option<char>,
    pub undone: Option<char>,
    pub done: Option<char>,
    pub need_work: Option<char>,
    pub urgent: Option<char>,
    pub recurring: Option<char>,
    pub pending: Option<char>,
    pub hold: Option<char>,
    pub cancelled: Option<char>,
}

impl Config {
    pub fn new() {
        let dir_path = dirs::config_dir().unwrap().join("dodder");
        let file_path = dir_path.join("dodder.toml");
        if !dir_path.exists() {
            fs::create_dir(&dir_path).unwrap();
        }
        if !file_path.exists() {
            let mut file = fs::File::create(&file_path).unwrap();
            let mut temp = String::new();
            writeln!(temp, "# dodder config file").unwrap();
            writeln!(temp, "# extension set default file extension").unwrap();
            writeln!(temp, r#"# extension = "md""#).unwrap();
            writeln!(temp, "# dir set dodder directory").unwrap();
            writeln!(
                temp,
                r#"
# global = "~/.local/share/dodder"
    "#
            )
            .unwrap();
            writeln!(temp, "# symbol set gtd status icons").unwrap();
            writeln!(
                temp,
                r#"
# [symbol]
# none = '󰄱'
# undone  = '󰄮'
# done = '󰄲'
# need_work = '󰱒'
# urgent = '⚠'
# recurring = '󰑖'
# pending = '󰏤'
# hold = '󰐊'
# cancelled = '󰜺'
    "#
            )
            .unwrap();

            std::io::Write::write_all(&mut file, temp.as_bytes()).unwrap();
        }
    }
    pub fn read() -> Config {
        let mut file = fs::File::open(
            dirs::config_dir()
                .unwrap()
                .join("dodder")
                .join("dodder.toml"),
        )
        .unwrap();
        let mut temp = String::new();
        file.read_to_string(&mut temp).unwrap();
        toml::from_str(&temp[..]).unwrap()
    }

    pub fn get_global_path(&self) -> PathBuf {
        if let Some(p) = &self.global {
            p.to_owned()
        } else {
            PathBuf::new()
                .join(dirs::data_dir().unwrap())
                .join("dodder")
        }
    }

    pub fn get_extension(&self) -> String {
        if let Some(s) = &self.extension {
            s.to_owned()
        } else {
            "md".to_string()
        }
    }

    pub fn get_symbols(&self) -> CGTD {
        if let Some(c) = &self.symbol {
            c.to_owned()
        } else {
            CGTD {
                none: None,
                undone: None,
                done: None,
                need_work: None,
                urgent: None,
                recurring: None,
                pending: None,
                hold: None,
                cancelled: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Config;

    #[test]
    fn parse_config() {
        let config = toml::from_str::<Config>(
            r#"
        extenstion = "norg"
        global = "~/.local/share/tree"
        [symbol]
        none = ''
        "#,
        )
        .unwrap();
        assert_eq!(
            config.get_global_path(),
            PathBuf::from("~/.local/share/tree")
        );
        assert_eq!(config.get_extension(), "norg".to_string());
        assert_eq!(config.get_symbols().none, Some(''));
    }
    #[test]
    fn create_read() {
        Config::new();
        let config = Config::read();
        assert_eq!(config.symbol, None);
        assert_eq!(config.global, None);
        assert_eq!(config.extension, None);
        assert_eq!(
            config.get_global_path(),
            PathBuf::from("~/.local/share/dodder")
        );
    }
}
