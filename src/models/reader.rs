use std::io::Read;
use std::path::PathBuf;
use std::fs::File;
use flate2::read::GzDecoder;

/// Given a path to the json.gz file,
pub fn deflate_to_contents(p: PathBuf) -> Option<String> {
    let mut f: File = match File::open(p) {
        Ok(f) => f,
        Err(..) => return None,
    };

    let mut s: String = String::new();
    f.read_to_string(&mut s).unwrap();
    let mut d = GzDecoder::new(s.as_bytes()).unwrap();
    let mut res: String = String::new();

    d.read_to_string(&mut res).unwrap();

    Some(res)
}

/// Each line in the data file corresponds into an entry
pub fn lines_of(p: PathBuf) -> Vec<String> {
    let v: Vec<String> = Vec::new();
    v
}

