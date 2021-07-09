use std::io::{Read};
use std::path::PathBuf;
use std::fs::File;
use flate2::read::GzDecoder;

/// Given a path to the json.gz file,
pub fn deflate_to_contents(p: PathBuf) -> Option<String> {
    let pp: PathBuf = p.clone();
    let ppstring: &str = pp.to_str().unwrap_or("[uncapable of unwraping]");

    let mut f: File = match File::open(p) {
        Ok(f) => f,
        Err(..) => return None,
    };

    let mut bytes: Vec<u8> = Vec::new();

    match f.read_to_end(&mut bytes) {
        Ok(..) => {},
        Err(e) => {
            println!("Problem loading file: {}", e);
            return None;
        },
    }

    let bb: &[u8] = bytes.as_ref();
    let mut d = GzDecoder::new(bb);
    let mut decomp: String = String::new();

    match d.read_to_string(&mut decomp) {
        Err(e) => {
            println!("Problem reading archive to string from error {}; archive was {}",
                     e, ppstring);
            return None;
        },
        Ok(..) => {},
    }

    Some(decomp)
}

/// Deflate and read each line. Each line in the data file corresponds into an entry
pub fn lines_of(p: PathBuf) -> Vec<String> {
    let data: String = match deflate_to_contents(p) {
        Some(v) => v,
        None => "".into(),
    };

    data.lines()
        .map(|e| e.into())
        .collect()
}
