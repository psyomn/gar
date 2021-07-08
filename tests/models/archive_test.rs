use gar::models::archive::ArchiveBuilder;

#[test]
fn archive_test() {
    let arch = ArchiveBuilder::new()
        .year(1000)
        .month(1)
        .day(1)
        .hour(1)
        .finalize();

    assert_eq!("1000-01-01-1.json.gz", arch.get_name());
}
