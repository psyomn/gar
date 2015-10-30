use rustc_serialize::json::Json;
use models::payloads::ShaElement;

#[derive(Debug)]
pub struct PushPayload {
    head: String, /* sha hash */
    refs: String, /* eg: refs/head/master */
    size: u64,
    shas: Vec<ShaElement>,
}

impl PushPayload {
    pub fn from_json(json: &Option<&Json>) -> Option<PushPayload> {
        None
    }
}

