use regex::Regex;
use rustc_serialize::json::Json;

use models::payloads::ShaElement;
use models::json_helpers::JsonHelper;

#[derive(Debug)]
pub struct PushPayload {
    head: String, /* sha hash */
    refs: String, /* eg: refs/head/master */
    size: u64,
    shas: Vec<ShaElement>,
}

impl PushPayload {
    pub fn from_json(json: Option<&Json>) -> Option<PushPayload> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let ref obj = json.unwrap().as_object().unwrap();

        let head = JsonHelper::string_or_empty(obj.get("head"));
        let refs = JsonHelper::string_or_empty(obj.get("ref"));
        let size = JsonHelper::number_or_zero(obj.get("size"));

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

    /// Given some text, check to see if any of the sha commits contain that text
    pub fn sha_elements_contain_text_of(&self, text: &str) -> bool {
        let re_txt: String = format!("(?i){}", text);
        let re: Regex = Regex::new(re_txt.as_ref()).unwrap();

        self.shas
            .iter()
            .map(|e| re.is_match(e.get_comment().as_ref()))
            .fold(false, |e,sum| sum || e)
    }
}

