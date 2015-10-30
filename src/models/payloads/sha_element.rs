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
}
