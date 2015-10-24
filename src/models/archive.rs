use std::io::Read;
use chrono::*;
use hyper::Client;
use hyper::client::{Response};
use hyper::header::Connection;

const GITHUT_ARCHIVE_URL: &'static str = "http://data.githubarchive.org/";

pub struct Archive {
    date: DateTime<UTC>,
}

impl Archive {

    pub fn new() -> Archive {
        Archive {
            date: UTC::now()
        }
    }

    /// Fetch the information of a specific archive. This will return something in memory, and will
    /// not make a local copy.
    pub fn fetch(&self) -> Vec<u8> {
        let url: String = format!("{}{}.json.gz",
            GITHUT_ARCHIVE_URL,
            self.date.format("%Y-%m-%d-%H"));

        let url_ref: &str = url.as_ref();

        let client: Client = Client::new();
        let mut resp: Response = client.get(url_ref)
            .header(Connection::close())
            .send().unwrap();

        let mut data: Vec<u8> = vec![];

        println!("Fetching {}", url_ref);
        match resp.read_to_end(&mut data) {
            Err(e) => { println!("{}", e); panic!(e)},
            _ => {},
        }
        data
    }

    /// Calls fetch, but also caches the data for future use
    pub fn fetch_and_store(&self) -> () {
        println!("TODO");
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
