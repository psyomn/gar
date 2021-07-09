use models::json_helpers::JsonHelper;
use rustc_serialize::json::*;

#[derive(Debug,Eq,PartialEq)]
pub struct WatchPayload {
    action: String,
}

impl WatchPayload {
    pub fn new(s: String) -> WatchPayload {
        WatchPayload {
            action: s,
        }
    }

    pub fn from_json(json: Option<&Json>) -> Option<WatchPayload> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let json = json.unwrap().as_object().unwrap();
        let action: String = JsonHelper::string_or_empty(json.get("action"));

        Some(WatchPayload { action: action } )
    }
}
