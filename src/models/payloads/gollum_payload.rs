use rustc_serialize::json::Json;
use models::payloads::page_element::PageElement;

#[derive(Debug)]
pub struct GollumPayload {
    pages: Vec<PageElement>,
}

impl GollumPayload {
    pub fn from_json(json: Option<&Json>) -> Option<GollumPayload> {
        if json.is_none() { return None }

        let ref obj = match json.unwrap().as_object() {
            None => return None,
            Some(v) => v,
        };

        let pages = match obj.get("pages") {
            Some(v) => v,
            None => return None,
        };

        let mut pes_vec: Vec<PageElement> = Vec::new();

        match pages {
            &Json::Array(ref vec) => {
                for el in vec.iter() {
                    /* Iterate over each page entry/object */
                    match PageElement::from_json(&Some(el)) {
                        None => continue,
                        Some(v) => pes_vec.push(v),
                    }
                }
            },
            _ => {},
        }

        Some(GollumPayload {
            pages: pes_vec,
        })
    }
}
