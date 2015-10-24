use toml;
use toml::Table;

use std::io::{Read, Write};

use std::fs;
use std::fs::File;
use std::fs::PathExt;

use std::env;
use std::path::PathBuf;

const NAME: &'static str = "gar";
const LONG_NAME: &'static str = "githubarchive";
const DATADIR: &'static str = "data";
const PREFIX: &'static str = ".config";
const CONFIG: &'static str = "gar.toml";

pub fn config_path() -> PathBuf {
    let home = match env::var("HOME") {
        Ok(v) => v,
        Err(e) => {
            println!("Could not get the HOME environment variable");
            panic!(e);
        },
    };
    let mut path = PathBuf::new();
    path.push(home);
    path.push(PREFIX);
    path.push(NAME);
    path
}

pub fn data_path() -> PathBuf {
    let mut base = config_path();
    base.push(DATADIR);
    base
}

pub fn config_file_path() -> PathBuf {
    let mut base = config_path();
    base.push(CONFIG);
    base
}

fn read_configuration_file() -> Table {
    let mut f: File = match File::open(config_file_path()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    let mut s: String = String::new();
    f.read_to_string(&mut s).unwrap();

    toml::Parser::new(&s).parse().unwrap()
}

pub fn caching_on() -> bool {
    let t: Table = read_configuration_file();
    match t.get(&"config".to_string()) {
        Some(v) => {
            match v {
                &toml::Value::Table(ref vv) => {
                    match vv.get(&"caching".to_string()) {
                        Some(s) => match s {
                            &toml::Value::String(ref s) => {
                                s == "yes"
                            },
                            _ => false,
                        },
                        None => false,
                    }
                },
                _ => false,
            }
        },
        None => false,
    }
}

pub fn data_exists(filename: &String) -> bool {
    let mut dpath: PathBuf = data_path();
    dpath.push(filename);
    dpath.exists()
}

/// Default things to run each time we go through the main entry point.
pub fn init() -> () {
    let cpath: PathBuf = config_path();
    let dpath: PathBuf = data_path();
    let config_file: PathBuf = config_file_path();

    if !cpath.exists() {
        println!("Config file path created for the first time");
        fs::create_dir_all(cpath.to_str().unwrap()).unwrap();
    }

    if !dpath.exists() {
        println!("Data path created for the first time");
        fs::create_dir_all(dpath.to_str().unwrap()).unwrap();
    }

    if !config_file.exists() {
        let mut t: Table = toml::Table::new();
        let mut caching_table: Table = toml::Table::new();

        caching_table.insert("caching".to_string(), toml::Value::String("yes".to_string()));
        t.insert("config".to_string(), toml::Value::Table(caching_table));

        let s: String = toml::encode_str(&t);

        let mut f: File = match File::create(config_file) {
            Ok(v)  => v,
            Err(e) => panic!("Could not open config file for writing {}", e),
        };

        println!("Writing configuration for the first time");
        f.write_all(s.as_bytes()).unwrap();
    }
}


