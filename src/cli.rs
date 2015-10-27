use chrono::*;

use models::archive::{Archive, ArchiveBuilder};
use config::*;

use std::fs;
use std::path::PathBuf;

use models::reader::*;
use models::repo::Repo;

/// Print the current version of GAR
pub fn version() -> () {
    println!("app version: {}", env!("CARGO_PKG_VERSION"));
}

fn parse_archive_date(date: String) -> Option<DateTime<UTC>> {
    let vals: Vec<&str> = date.split("-").collect::<Vec<&str>>();

    if vals.len() < 4 {
        println!("Error {:?}: You need to provide a date in the format of dd-mm-yyyy", date);
        return None;
    }

    #[inline]
    fn try_int_parse(s: Option<&str>) -> Option<u32> {
        match s {
            Some(v) => match v.parse::<u32>() {
                Ok(iv) => Some(iv),
                Err(..) => None,
            },
            None => None,
        }
    }

    let mut it = vals.into_iter();

    Some(UTC.ymd(
        try_int_parse(it.next()).unwrap() as i32,
        try_int_parse(it.next()).unwrap(),
        try_int_parse(it.next()).unwrap())
       .and_hms(
        try_int_parse(it.next()).unwrap(),
        0, 0))
}

fn parse_archive(date: String) -> Option<Archive> {
    let date = parse_archive_date(date).unwrap();

    let year  = date.year();
    let month = date.month();
    let day   = date.day();
    let hour  = date.hour();

    let mut a: Archive =
        ArchiveBuilder::new()
            .year(year)
            .month(month)
            .day(day)
            .hour(hour)
            .finalize();
    Some(a)
}

/// Same as fetch, but we're fetching a date range
pub fn fetch_rng(from: Option<String>, to: Option<String>) -> () {
    use time::Duration;

    fn match_and_parse(o: Option<String>) -> Option<DateTime<UTC>> {
        match o {
            Some(v) => parse_archive_date(v),
            None => return None,
        }
    }

    let fr = match_and_parse(from);
    let to = match_and_parse(to);

    if fr.is_none() || to.is_none() {
        println!("You need to supply both from, and to option flags");
    }

    let fr_v = fr.unwrap();
    let to_v = to.unwrap();

    if fr_v > to_v {
        println!("You need to supply valid date ranges");
        println!("From is after To in this case");
    }


    println!("Before duration, fr_v is: {}", fr_v);
    let d = Duration::minutes(60);
    let tmp = fr_v + d;
    println!("After duration, fr_v is: {}", tmp);

}

/// Argument provided should be in the form dd-mm-YYYY
pub fn fetch(s: String) -> () {
    let mut archive = match parse_archive(s) {
        Some(a) => a,
        None => panic!("Could not fetch archive"),
    };
    archive.fetch();
}

pub fn ls_data() -> () {
    let dpath: PathBuf = data_path();
    let paths = match fs::read_dir(dpath) {
        Ok(ps) => ps,
        Err(e) => panic!("Problem with data directory {}", e),
    };

    for p in paths {
        /* p is Option<Result<DirEntry>> */
        match p {
            Ok(v) => {
                let datafile_path = v.path();
                match datafile_path.to_str() {
                    Some(finv) => {
                        let size = match v.metadata() {
                            Ok(mv) => mv.len(),
                            Err(..) => 0,
                        };
                        println!("    {} [{}]", finv, size)
                    },
                    None => continue,
                }
            },
            Err(..) => continue,
        }
    }
}

/// Print the standard paths that the app uses.
pub fn show_paths() -> () {
    println!("Base: {:?}", config_path());
    println!("Data: {:?}", data_path());
    println!("Conf: {:?}", config_file_path());
}

pub fn find(vals: String) -> () {
    use std::fs::File;
    use walkdir::WalkDir;

    let p: PathBuf = data_path();
    let start = WalkDir::new(p);
    let mut v: Vec<String> = Vec::new();

    for entry in start.into_iter().filter_map(|e| e.ok()) {
        let eisf = File::open(entry.path()).ok().unwrap()
            .metadata().ok().unwrap().file_type().is_file();

        if eisf {
            println!("{}", entry.path().to_str().unwrap());
            let file_path: String = entry.path().to_str().unwrap().to_string();
            v.push(file_path);
        }
    }

    match v.pop() {
        Some(v) => {
            let first_line = lines_of(PathBuf::from(v)).iter().nth(0).unwrap().clone();
            println!("supplying {}", first_line);
            let repo = Repo::from_json(first_line);
            match repo {
                Some(v) => {
                    println!("{:#?}", v);
                },
                None => {
                    println!("Won't print anything");
                }
            }
        },
        None => println!("nothing to do"),
    }
}
