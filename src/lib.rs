#![feature(path_ext)]

//! This is a small tool that helps you interface with githubarchive.org
//! It provides utilities to fetch specific archives, or fetch a range of archives via date.
//!
//! To fetch a particular archive you need to do the following:
//!
//!     gar -f 2013-11-11-11
//!
//! To fetch a range of archives you need to do the following:
//!
//!     gar --fetch-rng --from 2013-10-10-13 --to 2014-11-11-14
//!
//! (WIP) To search for a particular feature, you can do the following:
//!
//!     gar --select url,name --where language:Rust
//!
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

