use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::fs::File;
use flate2::read::GzDecoder;

/// Given a path to the json.gz file,
pub fn deflate_to_contents(p: PathBuf) -> Option<String> {
    let mut stderr = io::stderr();
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
            writeln!(&mut stderr, "Problem loading file: {}", e).unwrap();
            return None;
        },
    }

    let bb: &[u8] = bytes.as_ref();
    let mut d = match GzDecoder::new(bb) {
        Ok(v) => v,
        Err(e) => panic!("Problem deflating: {}", e),
    };
    let mut decomp: String = String::new();

    match d.read_to_string(&mut decomp) {
        Err(e) => {
            writeln!(&mut stderr,
                     "Problem reading archive to string from error {}; archive was {}",
                     e, ppstring).unwrap();
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

