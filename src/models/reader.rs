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

    let mut bytes: Vec<u8> = Vec::new();

    f.read_to_end(&mut bytes).unwrap();

    let bb: &[u8] = bytes.as_ref();
    let mut d = GzDecoder::new(bb).unwrap();
    let mut decomp: String = String::new();

    let bytecount = d.read_to_string(&mut decomp).unwrap();

    Some(decomp)
}

/// Each line in the data file corresponds into an entry
pub fn lines_of(p: PathBuf) -> Vec<String> {
    let data: String = match deflate_to_contents(p) {
        Some(v) => v,
        None => "".into(),
    };

    data.lines()
        .map(|e| e.into())
        .collect()
}

