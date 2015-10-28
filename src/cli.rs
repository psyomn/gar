use chrono::*;
use walkdir::WalkDir;

use config::*;

use std::fs::File;
use std::fs;
use std::path::PathBuf;

use models::reader::*;
use models::repo::Repo;
use models::archive::{Archive, ArchiveBuilder};

/// Print the current version of GAR
pub fn version() -> () {
    println!("app version: {}", env!("CARGO_PKG_VERSION"));
}

/// Given a date in the format of "YYYY-m-d-h", parse the string into a DateTime<UTC> object
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

/// Given a date in string format, try to make it into an archive object, which can then be used to
/// fetch the data off the internets.
fn parse_archive(date: String) -> Option<Archive> {
    let date = match parse_archive_date(date) {
        Some(v) => v,
        None => return None,
    };

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

    let mut fr_v = fr.unwrap();
    let to_v = to.unwrap();

    if fr_v > to_v {
        println!("You need to supply valid date ranges");
        println!("From is after To in this case");
    }

    let one_hour = Duration::minutes(60);
    let mut date_strings: Vec<String> = Vec::new();

    while fr_v != to_v {
        /* And now generate the dates within the range by incrementing by hour */
        let s: String = format!("{}", fr_v.format("%Y-%m-%d-%H"));
        date_strings.push(s);
        fr_v = fr_v + one_hour;
    }

    for date in date_strings {
        fetch(date);
    }
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
    let v = vec![config_path(), data_path(), config_file_path()];
    v.into_iter().map(|e| ::print_green(format!("  {}\n", e.to_str().unwrap()).as_ref()))
        .collect::<Vec<()>>();
}

/// Given a select, and where clause, match and find against those.
pub fn find(from: Option<String>, to: Option<String>,
            selects: Option<String>, wheres: Option<String>) -> () {
}

/// This will look into the ~/.config/gar/data folder, and match the filenames against the given
/// dates. If the match is successful, then the path to that archive is returned.
fn choose_files_from_dates(from: Option<String>, to: Option<String>) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = get_data_file_paths();
    if from.is_some() && to.is_some() {
        v.into_iter().collect()
    }
    else if from.is_some() && to.is_none() {
        v.into_iter().collect()
    }
    else if from.is_none() && to.is_some() {
        v.into_iter().collect()
    }
    else {
        /* Both none - match all */
        v.into_iter().collect()
    }
}

fn get_data_file_paths() -> Vec<PathBuf> {
    let p: PathBuf = data_path();
    let start = WalkDir::new(p);
    let mut v: Vec<PathBuf> = Vec::new();

    for entry in start.into_iter().filter_map(|e| e.ok()) {
        let eisf = File::open(entry.path()).ok().unwrap()
            .metadata().ok().unwrap().file_type().is_file();
        if eisf {
            v.push(entry.path().to_path_buf());
        }
    }

    v
}
