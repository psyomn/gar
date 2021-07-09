use gar::models::event_type::EventType;

#[test]
fn event_type_from_string_test() {
    assert_eq!(EventType::Create, "CreateEvent".parse::<EventType>().unwrap());
    assert_eq!(EventType::Download, "DownloadEvent".parse::<EventType>().unwrap());

    let unknown_msg = "hahahaha nope and nope".to_string();
    assert_eq!(EventType::Unknown(unknown_msg.clone()),
               unknown_msg.parse::<EventType>().unwrap());
}
