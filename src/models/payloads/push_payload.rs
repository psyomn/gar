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

        let none: &Vec<Json> = &vec![];
        let size = match obj.get("size") {
            Some(v) => match *v {
                Json::U64(v) => v,
                _ => 0,
            },
            None => 0,
        };

        let mut distinct = false;
        let mut shas: Vec<ShaElement> = Vec::new();

        // TODO not too happy about this.
        // TODO this should be moved inside sha_element
        match obj.get("shas") {
            Some(v) => match v {
                &Json::Array(ref a) => {
                    for (ix, ref sha) in a.iter().enumerate() {
                        let mut sha_str: String = String::new();
                        let mut email_str: String = String::new();
                        let mut desc_str: String = String::new();
                        let mut user_str: String = String::new();

                        match *sha {
                             &Json::String(ref s) => match ix {
                                 0 => sha_str   = s.clone(),
                                 1 => email_str = s.clone(),
                                 2 => desc_str  = s.clone(),
                                 3 => user_str  = s.clone(),
                                 _ => {},
                             },
                             &Json::Boolean(b) => distinct = b,
                             _ => {},
                        }

                        shas.push(
                            ShaElement::from_values(
                                sha_str,
                                email_str,
                                desc_str,
                                user_str,
                                distinct));
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

