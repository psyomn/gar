extern crate gar;

use std::path::PathBuf;

// relative to tests/
const TEST_DIR: &str = "tests";
const DATA_DIR: &str = "fixtures";
const DATA_SAMPLE: &str = "2021-01-01-1.json.gz";

fn fixture_path(s :&str) -> PathBuf {
    let mut pb = PathBuf::new();
    pb.push(TEST_DIR);
    pb.push(DATA_DIR);
    pb.push(s);
    pb
}

mod models {
    mod archive_test;
    mod event_test;
    mod event_type_test;
    mod language_test;
}
