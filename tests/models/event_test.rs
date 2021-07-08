use gar::models::event::Event;

#[test]
fn event_default_test() {
    let event: Event = Default::default();

    assert_eq!(0, event.get_gh_id());
    assert_eq!("", event.get_name());
    assert_eq!("", event.get_description());
}
