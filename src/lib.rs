#![feature(path_ext)]

//! This is a small tool that helps you interface with githubarchive.org
//! It provides utilities to fetch specific archives, or fetch a range of archives via date.
//! You can then run some semi-complex queries on the downloaded archives, which are still in gz
//! format. Try running `help` to see information of each command, and subcommands.
//!
//!     gar 0.2.0
//!     Simon psyomn Symeonidis <lethaljellybean@gmail.com>
//!     Github Archive interfacing and querying tool
//!
//!     USAGE:
//!     	gar [FLAGS] [SUBCOMMAND]
//!
//!     FLAGS:
//!         -h, --help       Prints help information
//!         -v, --version    show the current version
//!
//!     SUBCOMMANDS:
//!         fetch    for fetching singular files
//!         help     Prints this message
//!         query    for running queries on the retrieved data
//!         show     for printing different program information
//!
//! # Fetching
//!
//! Let's take a look at simple fetching commands:
//!
//!     gar-fetch
//!     for fetching singular files
//!
//!     USAGE:
//!     	gar fetch [FLAGS] [OPTIONS] [SUBCOMMAND]
//!
//!     FLAGS:
//!         -h, --help       Prints help information
//!         -V, --version    Prints version information
//!
//!     OPTIONS:
//!             --file <file>    the date in YYYY-mm-dd-h format
//!
//!     SUBCOMMANDS:
//!         help     Prints this message
//!         range    for fetching from certain dates
//!
//! You can fetch one file this way:
//!
//!     gar fetch --file 2014-1-1-1
//!
//! Or if you want a specific range:
//!
//!     gar fetch range --from 2013-1-1-1 --to 2013-1-5-1
//!
//! # Querying
//!
//! You can run simple queries this way:
//!
//!     gar-query
//!     for running queries on the retrieved data
//!
//!     USAGE:
//!     	gar query [FLAGS] [OPTIONS]
//!
//!     FLAGS:
//!         -h, --help       Prints help information
//!         -V, --version    Prints version information
//!
//!     OPTIONS:
//!         -f, --from <from>            specify query date in YYYY-mm-dd-h format
//!         -s, --select <select>        specify which fields to output
//!         -m, --template <template>    specify handlebar template for output
//!         -t, --to <to>                specify query date in YYYY-mm-dd-h format
//!         -w, --where <where>          specify selection constraints
//!
//! Here is an example of a query:
//!
//!     gar query --where language:Rust,type:create
//!
//! This will search for all events, and select only the events where the repository is of the Rust
//! language, and the type of event is a CreateEvent. You can also specify time constraints with to
//! and from:
//!
//!     gar query --where language:Rust,type:create --from 2013-1-1-1 --to 2013-1-5-1
//!
//! And as you noticed you can also provide a type of event, and language using the `--where` clause.
//! The way you do this, is by providing a label, delimited with a colon `:` and provide the value.
//! For example:
//!
//!     language:Rust
//!
//! Satisfies this query. You can add more constraints by delimiting them with a comma ','. The
//! relevance of a comma in this case is as if it's a logical `AND`. As you previously saw:
//!
//!     language:Rust,type:create
//!
//! These are the event types you can capture using these labels:
//!
//!     type:<event-type> where <event-type> is:
//!
//!         create
//!         commit_comment
//!         delete
//!         deployment
//!         deployment_status
//!         download
//!         follow
//!         fork
//!         fork_apply
//!         gist
//!         gollum
//!         issue_comment
//!         issues
//!         member
//!         membership
//!         page_build
//!         public
//!         pull_request
//!         pull_request_review_comment
//!         push
//!         release
//!         repository
//!         status
//!         team_add
//!         watch

extern crate rustc_serialize;
extern crate time;

extern crate toml;
extern crate chrono;
extern crate hyper;
extern crate regex;
extern crate term;
extern crate flate2;
extern crate walkdir;
extern crate handlebars;

pub mod models;
pub mod config;
pub mod cli;

#[inline]
fn print_yellow(s: &str) -> () {
    generic_print(s, term::color::YELLOW);
}

#[inline]
fn print_green(s: &str) -> () {
    generic_print(s, term::color::GREEN);
}

#[inline]
fn print_red(s: &str) -> () {
    generic_print(s, term::color::RED);
}

fn print_magenta(s: &str) -> () {
    generic_print(s, term::color::MAGENTA);
}

#[inline]
fn generic_print(s: &str, col: term::color::Color) -> () {
    let mut t = term::stdout().unwrap();
    t.fg(col).unwrap();
    write!(t, "{}", s).unwrap();
    t.reset().unwrap();
}

fn vec_contains<T: PartialEq>(v: &Vec<T>, t: &T) -> bool {
    for e in v {
        if e == t { return true }
    }
    false
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_contains() -> () {
        let vs: Vec<String> = vec!["potato".into(), "yotato".into(), "motato".into()];
        let vu32: Vec<u32> = vec![1,2,3,4,5,6];
        let vi32: Vec<i32> = vec![1,2,3,4,5];
        let jon: String = "jon".into();

        assert!(::vec_contains::<String>(&vs, &"potato".into()));
        assert!(::vec_contains::<u32>(&vu32, &2));
        assert!(::vec_contains::<i32>(&vi32, &5));

        assert!(!::vec_contains::<i32>(&vi32, &122));
        assert!(!::vec_contains::<u32>(&vu32, &123123));
        assert!(!::vec_contains::<String>(&vs, &jon));
    }
}

