extern crate walkdir;
extern crate getopts;
extern crate gar;

use gar::cli;
use gar::config;

use std::env;
use getopts::Options;

fn main() {
    config::init();

    let args: Vec<String> = env::args().collect();
    let opts = match make_opts().parse(&args[1..]) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    if opts.opt_present("show-paths") { cli::show_paths(); return }
    if opts.opt_present("ls-data") { cli::ls_data(); return }
    if opts.opt_present("v") { cli::version(); return }
    if opts.opt_present("h") {
        help(args[0].clone().as_ref(), make_opts());
        return;
    }
    if opts.opt_present("f") {
        /* gar --fetch */
        match opts.opt_str("f") {
            Some(v) => cli::fetch(v),
            None => panic!("You need to supply a date for fetch"),
        }
        return;
    }

    if opts.opt_present("fetch-rng") {
        cli::fetch_rng(opts.opt_str("from"), opts.opt_str("to"));
        return;
    }

    if opts.opt_present("select") {
        let selects: Option<String> = opts.opt_str("select");
        let wheres: Option<String> =  opts.opt_str("where");
        let from: Option<String> = opts.opt_str("from");
        let to: Option<String> = opts.opt_str("to");
        let template: Option<String> = opts.opt_str("template");
        let simple_print: bool = opts.opt_present("simple-print");
        cli::find(from, to, selects, wheres, template);
        return;
    }

    println!("run gar -h for help");
}

fn make_opts() -> Options {
    let mut options: Options = Options::new();

    options.optopt("f", "fetch", "FETCH", "fetch a particular archive");
    options.optopt("", "find", "FEATURE", "feature to look for.");
    options.optflag("h", "help", "print this");
    options.optflag("v", "version", "show the version");
    options.optflag("", "show-paths", "show the paths that the application uses");
    options.optflag("", "ls-data", "print the data files");
    options.optflag("", "fetch-rng", "use this with from, to opt flags to fetch a range of archives");
    options.optopt("", "from", "FROM", "specify date from (use with fetch)");
    options.optopt("", "to", "TO", "specify date to (use with fetch)");
    options.optopt("", "template", "TEMPLATE", "specify the handlebar template to use");
    options.optflag("", "simple-print", "simple print results");

    // eg: gar --select url,name --where language:Rust
    options.optopt("", "select", "SELECT", "select specific fields of matched repos");
    options.optopt("", "where", "WHERE", "constraints on the select");

    options
}



/// Print the options menu
pub fn help(program: &str, opts: Options) -> () {
    let brief = format!("{} [options]", program);
    print!("{}", opts.usage(&brief));
}
