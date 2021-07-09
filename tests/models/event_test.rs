use crate::DATA_SAMPLE;

use gar::models::event::Event;

#[test]
fn default_test() {
    let event: Event = Default::default();

    assert_eq!(0, event.get_gh_id());
    assert_eq!("", event.get_name());
    assert_eq!("", event.get_description());
}

#[test]
fn fixture_test() {
    let events = Event::from_path(::fixture_path(DATA_SAMPLE));
    assert_eq!(1000, events.len());
}

#[test]
fn json_object_parsing_test() {
    let json_string = r#"
        {
         "id":"14684219584",
         "type":"CreateEvent",
         "actor":{
           "id":69695756,
           "login":"someone",
           "display_login":"someone_login",
           "gravatar_id":"",
           "url":"https://api.github.com/users/someone",
           "avatar_url":"https://avatars.githubusercontent.com/u/696?"
         },
         "repo":{
           "id":325897057,
           "name":"someone/somethingrepo",
           "url":"https://api.github.com/repos/someone/totally-a-repository"
         },
         "payload":{
           "ref":null,
           "ref_type":"repository",
           "master_branch":"master",
           "description":"this is an interesting description",
           "pusher_type":"user"
         },
         "public":true,
         "created_at":"2021-01-01T01:00:00Z"
        }"#;

    let event_result = Event::into_json_object(&json_string).unwrap();

    println!("{:?}", event_result);
}
