use rustc_serialize::json::Json;

#[derive(Debug)]
pub struct DeletePayload {
    ref_tag: String,
    ref_tag_type: String,
}

impl DeletePayload {
    pub fn from_json(json: &Option<&Json>) -> Option<DeletePayload> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let ref obj = json.unwrap().as_object().unwrap();

        let ref_tag = match obj.get("ref") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        let ref_tag_type = match obj.get("ref_type") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        Some(DeletePayload {
            ref_tag: ref_tag,
            ref_tag_type: ref_tag_type,
        })
    }
}
