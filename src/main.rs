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
        (@subcommand query =>
            (about: "for running queries on the retrieved data")
            (@arg from:   -f --from +takes_value "specify query date in YYYY-mm-dd-h format")
            (@arg to:     -t --to +takes_value "specify query date in YYYY-mm-dd-h format")
            (@arg select: -s --select +takes_value "specify which fields to output")
            (@arg where:  -w --where +takes_value "specify selection constraints")
            (@arg template: -m --template +takes_value "specify handlebar template for output")
        )
    ).get_matches();

    if let Some(matches) = matches.subcommand_matches("fetch") {
        if matches.is_present("file") {
            let filename = matches.value_of("file").unwrap();
            cli::fetch(filename.into());
            return;
        }
        if let Some(matches) = matches.subcommand_matches("range") {
            let from = matches.value_of("from").map(|e| e.into());
            let to = matches.value_of("to").map(|e| e.into());
            cli::fetch_rng(from, to);
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

    if let Some(matches) = matches.subcommand_matches("query") {
        let selects: Option<String> = matches.value_of("select").map(|e| e.into());
        let wheres: Option<String> = matches.value_of("where").map(|e| e.into());
        let from: Option<String> = matches.value_of("from").map(|e| e.into());
        let to: Option<String> = matches.value_of("to").map(|e| e.into());
        let template: Option<String> = matches.value_of("template").map(|e| e.into());
        cli::find(from, to, selects, wheres, template);
        return;
    }
}
