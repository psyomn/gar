use chrono::*;
use hyper::Client;
use hyper::client::{Response};
use hyper::header::Connection;

use std::path::PathBuf;
use std::fs::File;
use std::io::{Read, Write};

use config;

const GITHUT_ARCHIVE_URL: &'static str = "http://data.githubarchive.org/";

/// An archive is a file object - not to be confused with repos, or things that will give us access
/// to data.
#[derive(Debug)]
pub struct Archive {
    date: DateTime<UTC>,
    data: Vec<u8>,
    name: String,
}

pub struct ArchiveBuilder {
    year: i32,
    month: u32,
    day: u32,
    hour: u32
}

impl Archive {

    pub fn new(y: i32, m: u32, d: u32, h: u32) -> Archive {
        let d = UTC.ymd(y, m, d).and_hms(h, 0, 0);
        let n = Archive::make_title(d);

        Archive {
            date: d,
            data: vec![],
            name: n,
        }
    }

    fn make_date(d: DateTime<UTC>) -> String {
        d.format("%Y-%m-%d-%-k").to_string()
    }

    fn make_title(d: DateTime<UTC>) -> String {
        format!("{}.json.gz", Archive::make_date(d))
    }

    /// Fetch the information of a specific archive. This will return something in memory, and will
    /// not make a local copy.
    pub fn fetch(&mut self) -> () {
        let title: String = Archive::make_title(self.date);

        if config::data_exists(&title) {
            ::print_yellow(format!("Data {} exists in cache - skip\n", title).as_ref());
            return;
        }

        let url: String = format!("{}{}", GITHUT_ARCHIVE_URL, title);
        let url_ref: &str = url.as_ref();

        let client: Client = Client::new();
        let mut resp: Response = client.get(url_ref)
            .header(Connection::close())
            .send().unwrap();

        let mut data: Vec<u8> = vec![];

        print!("Fetching {}", url_ref);
        match resp.read_to_end(&mut data) {
            Err(e) => { println!("{}", e); panic!(e)},
            _ => {},
        }

        self.data = data;

        if &self.data[0..5] == b"<?xml" {
            ::print_magenta(format!("\nNo such info found on server ({})\n", url).as_ref());
            return;
        }
        else {
            ::print_green(format!(" ok\n").as_ref());
        }

        if config::caching_on() { self.store() }
    }

    pub fn store(&self) -> () {
        let mut base: PathBuf = config::data_path();
        let s: String = match base.clone().to_str() {
            Some(v) => v.to_string(),
            None => return,
        };

        base.push(self.name.clone());

        let mut f: File = match File::create(base) {
            Ok(v) => v,
            Err(e) => {
                println!("Problem opening caching file @ {:?}", s);
                println!("{}", e);
                return;
            },
        };

        f.write_all(&self.data).unwrap();
    }

    /// Set the year of the archive we're interested in
    pub fn set_year(&mut self, year: i32) -> () {
        self.date = UTC.ymd(year, self.date.month(), self.date.day())
                       .and_hms(9, 0, 0);
    }

    /// Set the month of the archive we're interested in
    pub fn set_month(&mut self, month: u32) -> () {
        self.date = UTC.ymd(self.date.year(), month, self.date.day()).and_hms(9, 0, 0);
    }

    /// Set the day of the archive we're interested in
    pub fn set_day(&mut self, day: u32) -> () {
        self.date = UTC.ymd(self.date.year(), self.date.month(), day).and_hms(9, 0, 0);
    }

    /// Set the hour of the archive we're interested in
    pub fn set_hour(&mut self, h: u32) -> () {
        self.date = UTC.ymd(self.date.year(), self.date.month(), self.date.day()).and_hms(h, 0, 0);
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

