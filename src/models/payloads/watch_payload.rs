use rustc_serialize::json::*;

#[derive(Debug)]
pub struct WatchPayload {
    action: String,
}

impl WatchPayload {
    pub fn new(s: String) -> WatchPayload {
        WatchPayload {
            action: s,
        }
    }

    pub fn from_json(json: &Option<&Json>) -> Option<WatchPayload> {
        if json.is_none() { return None }

        let ref preobj = *json.unwrap();

        if !preobj.is_object() { return None }

        let json = preobj.as_object().unwrap();

        let action: String = match json.get("action") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        Some(WatchPayload { action: action } )
    }
}
