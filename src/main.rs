extern crate getopts;
extern crate gar;

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
        cli::help(args[0].clone().as_ref(), make_opts());
        return;
    }
    if opts.opt_present("f") {
        let val: String = match opts.opt_str("f") {
            Some(v) => v,
            None => panic!("You need to supply a date for fetch"),
        };
        cli::fetch(val);
        return;
    }
}

fn make_opts() -> Options {
    let mut options: Options = Options::new();

    options.optopt("f", "fetch", "FETCH", "fetch a particular archive");
    options.optflag("h", "help", "print this");
    options.optflag("v", "version", "show the version");
    options.optflag("", "show-paths", "show the paths that the application uses");
    options.optflag("", "ls-data", "print the data files");

    options
}

mod cli {
    use gar::models::archive::{Archive, ArchiveBuilder};
    use gar::config::*;
    use getopts::Options;

    use std::fs;
    use std::path::PathBuf;

    /// Print the current version of GAR
    pub fn version() -> () {
        println!("app version: {}", env!("CARGO_PKG_VERSION"));
    }

    /// Print the options menu
    pub fn help(program: &str, opts: Options) -> () {
        let brief = format!("{} [options]", program);
        print!("{}", opts.usage(&brief));
    }

    /// Argument provided should be in the form dd-mm-YYYY
    pub fn fetch(s: String) -> () {
        let vals: Vec<&str> = s.split("-").collect::<Vec<&str>>();

        if vals.len() < 4 {
            println!("You need to provide a date in the format of dd-mm-yyyy");
            return;
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
        let year  = try_int_parse(it.next());
        let month = try_int_parse(it.next());
        let day   = try_int_parse(it.next());
        let hour  = try_int_parse(it.next());

        for x in vec![year, month, day, hour].iter() {
            match *x {
                None => return,
                _    => {},
            }
        }

        let mut a: Archive =
            ArchiveBuilder::new()
                .year(year.unwrap() as i32)
                .month(month.unwrap())
                .day(day.unwrap())
                .hour(hour.unwrap())
                .finalize();

        a.fetch();
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
}
