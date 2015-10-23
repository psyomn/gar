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
