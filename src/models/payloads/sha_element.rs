use rustc_serialize::json::*;

#[derive(Debug,Eq,PartialEq)]
pub struct ShaElement {
    sha: String,
    email: String,
    comment: String,
    author: String,
    distinct: bool,
}

impl ShaElement {
    pub fn from_json(json: &Option<&Json>) -> Option<ShaElement> {
        if json.is_none() { return None }

        let mut sha_str: String = String::new();
        let mut email_str: String = String::new();
        let mut desc_str: String = String::new();
        let mut user_str: String = String::new();
        let mut distinct: bool = false;

        let ref sha = json.unwrap();

        match *sha {
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


        Some(ShaElement {
            sha: sha_str,
            email: email_str,
            comment: desc_str,
            author: user_str,
            distinct: distinct,
        })
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

    pub fn get_comment(&self) -> &String {
        &self.comment
    }
}
