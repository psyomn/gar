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
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let ref obj = json.unwrap().as_object().unwrap();

        let head = match obj.get("head") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        let refs = match obj.get("ref") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        let size = match obj.get("size") {
            Some(v) => match *v {
                Json::U64(v) => v,
                _ => 0,
            },
            None => 0,
        };

        let mut shas: Vec<ShaElement> = Vec::new();

        // TODO not too happy about this.
        match obj.get("shas") {
            Some(v) => match v {
                &Json::Array(ref a) => {
                    for ref sha in a.iter() {
                        match ShaElement::from_json(&Some(sha)) {
                            Some(v) => shas.push(v),
                            None => continue,
                        }
                    }
                },
                _ => {},
            },
            None => {},
        }

        Some(PushPayload {
            head: head,
            refs: refs,
            size: size,
            shas: shas,
        })
    }
}

