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

        let mut distinct = false;
        let mut shas: Vec<ShaElement> = Vec::new();

        // TODO not too happy about this.
        // TODO this should be moved inside sha_element
        match obj.get("shas") {
            Some(v) => match v {
                &Json::Array(ref a) => {
                    for ref sha in a.iter() {
                        let mut sha_str: String = String::new();
                        let mut email_str: String = String::new();
                        let mut desc_str: String = String::new();
                        let mut user_str: String = String::new();

                        match *sha {
                             // &Json::String(ref s) => match ix {
                             // },
                             // &Json::Boolean(b) => distinct = b,
                             &Json::Array(ref vec) => {
                                 for (ix, val) in vec.iter().enumerate() {
                                     match (ix,val) {
                                         (0, &Json::String(ref s)) => sha_str   = s.clone(),
                                         (1, &Json::String(ref s)) => email_str = s.clone(),
                                         (2, &Json::String(ref s)) => desc_str  = s.clone(),
                                         (3, &Json::String(ref s)) => user_str  = s.clone(),
                                         (_, &Json::Boolean(b))    => distinct  = b,
                                         _ => {},
                                     }
                                 }
                             },
                             _ => {}
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

