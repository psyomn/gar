#![feature(path_ext)]

extern crate rustc_serialize;
extern crate time;

extern crate toml;
extern crate chrono;
extern crate hyper;
extern crate regex;
extern crate term;
extern crate flate2;
extern crate walkdir;

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
    generic_print(s, term::color::GREEN);
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

