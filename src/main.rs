extern crate walkdir;
extern crate gar;
#[macro_use] extern crate clap;

use gar::cli;
use gar::config;

fn main() {
    config::init();

    let matches = clap_app!(myapp =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Simon psyomn Symeonidis <lethaljellybean@gmail.com>")
        (about: "Github Archive interfacing and querying tool")
        (@arg version: -v --version "show the current version")
        (@subcommand show =>
            (about: "for printing different program information")
            (@arg data: -d --data "shows tha data folder")
            (@arg paths: -p --paths "show the paths the application uses")
        )
        (@subcommand fetch =>
            (about: "for fetching singular files")
            (@arg file: --file +takes_value "the date in YYYY-mm-dd-h format")
            (@subcommand range =>
                (about: "for fetching from certain dates")
                (@arg from: -f --from +takes_value "the date in YYYY-mm-dd-h format")
                (@arg to:   -t --to   +takes_value "the date in YYYY-mm-dd-h format")
            )
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("fetch") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap();
            return;
        }
        if let Some(matches) = matches.subcommand_matches("range") {
            let from = matches.value_of("from");
            let to = matches.value_of("to");
            return;
        }
    }

    if let Some(matches) = matches.subcommand_matches("show") {
        /* gar show --data
         * gar show --paths */
        if matches.is_present("data") {
            cli::ls_data();
            return;
        }
        if matches.is_present("paths") {
            cli::show_paths();
            return;
        }
    }

    if matches.is_present("version") {
        cli::version();
        return;
    }

    // TODO: Bellow shall be migrated to clap
    //
    // if opts.opt_present("f") {
    //     /* gar --fetch */
    //     match opts.opt_str("f") {
    //         Some(v) => cli::fetch(v),
    //         None => panic!("You need to supply a date for fetch"),
    //     }
    //     return;
    // }

    // if opts.opt_present("fetch-rng") {
    //     cli::fetch_rng(opts.opt_str("from"), opts.opt_str("to"));
    //     return;
    // }

    // if opts.opt_present("select") {
    //     let selects: Option<String> = opts.opt_str("select");
    //     let wheres: Option<String> =  opts.opt_str("where");
    //     let from: Option<String> = opts.opt_str("from");
    //     let to: Option<String> = opts.opt_str("to");
    //     let template: Option<String> = opts.opt_str("template");
    //     let simple_print: bool = opts.opt_present("simple-print");
    //     cli::find(from, to, selects, wheres, template);
    //     return;
    // }

    // println!("run gar -h for help");
}
