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
        if opts.opt_present("from") == opts.opt_present("to") {
            match opts.opt_present("from") || opts.opt_present("to") {
                true => {
                    /* gar --fetch --from X --to Y */
                    cli::fetch_rng(opts.opt_str("from"),
                                   opts.opt_str("to"));
                },
                false =>  {
                    let val: String = match opts.opt_str("f") {
                        Some(v) => v,
                        None => panic!("You need to supply a date for fetch"),
                    };
                    cli::fetch(val);
                },
            }
        }
        else {
            println!("Currently, you need to specify both from, and to dates, if\
            you choose to supply dates!");
        }
        return;
    }
    if opts.opt_present("find") {
        match opts.opt_str("find") {
            Some(v) => {
                cli::find(v);
                return;
            }
            None => panic!("You need to provide argument(s) in the form of <feature>:<value>,+"),
        }
    }
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

    options
}



/// Print the options menu
pub fn help(program: &str, opts: Options) -> () {
    let brief = format!("{} [options]", program);
    print!("{}", opts.usage(&brief));
}
