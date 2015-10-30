use rustc_serialize::json::*;

#[derive(Debug)]
pub struct ShaElement {
    sha: String,
    email: String,
    comment: String,
    author: String,
    distinct: bool,
}

impl ShaElement {
    pub fn from_json(json: &Option<Json>) -> Option<ShaElement> {
        None
    }

    pub fn from_values(s: String, e: String, c: String, a: String,
                       d: bool) -> ShaElement {
        ShaElement {
            sha: s,
            email: e,
            comment: c,
            author: a,
            distinct: d,
        }
    }
}

