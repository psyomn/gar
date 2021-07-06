use chrono::*;

use std::fs::File;
use std::fmt;
use std::path::PathBuf;

use std::io::Write;

use config;

const GITHUT_ARCHIVE_URL: &str = "https://data.githubarchive.org/";

// TODO might be worth to break into FetchErrors and FetchOks
pub enum FetchStatus {
    Success,
    Cached,
    FailFetch,
    NotFound,
    CantCreateCacheFile,
    CantWriteCacheFile,
    ResourceUnavailable,
}

impl fmt::Display for FetchStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FetchStatus::Success => write!(f, "success"),
            FetchStatus::Cached => write!(f, "cached"),
            FetchStatus::FailFetch => write!(f, "failed to fetch"),
            FetchStatus::NotFound => write!(f, "not found"),
            FetchStatus::CantCreateCacheFile => write!(f, "cant create cache file"),
            FetchStatus::CantWriteCacheFile => write!(f, "cant write cache file"),
            FetchStatus::ResourceUnavailable => write!(f, "resource unavailable"),
        }
    }
}

#[derive(Debug)]
pub struct Archive {
    date: DateTime<Utc>,
    data: Vec<u8>,
    name: String,
}

pub struct ArchiveBuilder {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
}

impl Archive {
    pub fn new(year: i32, month: u32, day: u32, hour: u32) -> Archive {
        let d = Utc.ymd(year, month, day).and_hms(hour, 0, 0);
        let n = Archive::make_title(d);

        Archive {
            date: d,
            data: vec![],
            name: n,
        }
    }

    fn make_date(d: DateTime<Utc>) -> String {
        d.format("%Y-%m-%d-%-k").to_string()
    }

    fn make_title(d: DateTime<Utc>) -> String {
        format!("{}.json.gz", Archive::make_date(d))
    }

    fn fetch_raw(url: &String) -> Result<Vec<u8>, FetchStatus> {
        let response = match attohttpc::get(&url).send() {
            Ok(v) => v,
            Err(_) => return Err(FetchStatus::FailFetch),
        };

        if !response.is_success() {
            // TODO: may be more cases here, but this will have to do
            //   for now. Apparently attohttpc actually uses hyperium,
            //   or some parts of it
            return Err(FetchStatus::NotFound);
        }

        match response.bytes() {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(FetchStatus::ResourceUnavailable),
        }
    }

    pub fn fetch(&mut self) -> Result<FetchStatus, FetchStatus> {
        let title: String = Archive::make_title(self.date);

        if config::data_exists(&title) {
            return Ok(FetchStatus::Cached)
        }

        let url: String = format!("{}{}", GITHUT_ARCHIVE_URL, title);

        match Archive::fetch_raw(&url) {
            Ok(value) => self.data = value,
            Err(_) => return Err(FetchStatus::FailFetch),
        }

        if config::caching_on() { return self.store() }

        Ok(FetchStatus::Success)
    }

    pub fn store(&self) -> Result<FetchStatus, FetchStatus> {
        let mut base: PathBuf = config::data_path();

        base.push(self.name.clone());

        let mut f: File = match File::create(base) {
            Ok(v) => v,
            Err(_) => return Err(FetchStatus::CantCreateCacheFile),
        };

        match f.write_all(&self.data) {
            Ok(_) => Ok(FetchStatus::Success),
            Err(_) => Err(FetchStatus::CantWriteCacheFile),
        }
    }

    pub fn set_year(&mut self, year: i32) -> () {
        self.date = Utc.ymd(year, self.date.month(), self.date.day())
                       .and_hms(9, 0, 0);
    }

    pub fn set_month(&mut self, month: u32) -> () {
        self.date = Utc.ymd(self.date.year(), month, self.date.day()).and_hms(9, 0, 0);
    }

    pub fn set_day(&mut self, day: u32) -> () {
        self.date = Utc.ymd(self.date.year(), self.date.month(), day).and_hms(9, 0, 0);
    }

    pub fn set_hour(&mut self, h: u32) -> () {
        self.date = Utc.ymd(self.date.year(), self.date.month(), self.date.day()).and_hms(h, 0, 0);
    }
}

impl fmt::Display for Archive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Archive::make_date(self.date))
    }
}

impl ArchiveBuilder {
    pub fn new() -> ArchiveBuilder {
        ArchiveBuilder {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
        }
    }

    pub fn year(&mut self, y: i32) -> &mut ArchiveBuilder {
        self.year = y;
        self
    }

    pub fn month(&mut self, m: u32) -> &mut ArchiveBuilder {
        self.month = m;
        self
    }

    pub fn day(&mut self, d: u32) -> &mut ArchiveBuilder {
        self.day = d;
        self
    }

    pub fn hour(&mut self, h: u32) -> &mut ArchiveBuilder {
        self.hour = h;
        self
    }

    pub fn finalize(&self) -> Archive {
        Archive::new(
            self.year,
            self.month,
            self.day,
            self.hour)
    }
}
