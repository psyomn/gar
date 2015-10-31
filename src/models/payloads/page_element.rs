use rustc_serialize::json::Json;

/// This is one of the elements found within the GollumEvent payload
#[derive(Debug)]
pub struct PageElement {
    action: String,
    html_url: String,
    page_name: String,
    sha: String,
    summary: Option<String>,
}

fn string_or_empty(s: Option<&Json>) -> String {
    match s {
        Some(v) => match v {
            &Json::String(ref s) => s.clone(),
            _ => "".into(),
        },
        None => "".into()
    }
}

impl PageElement {
    pub fn from_json(json: &Json) -> Option<PageElement> {
        if !json.is_object() { return None }

        let obj = json.as_object().unwrap();

        let action    = string_or_empty(obj.get("action"));
        let html_url  = string_or_empty(obj.get("html_url"));
        let page_name = string_or_empty(obj.get("page_name"));
        let sha       = string_or_empty(obj.get("sha"));

        let summary: Option<String> = match obj.get("summary") {
            Some(v) => match *v {
                Json::Null => None,
                Json::String(ref s) => Some(s.clone()),
                _ => None,
            },
            None => None,
        };

        Some(PageElement {
            action: action,
            html_url: html_url,
            page_name: page_name,
            sha: sha,
            summary: summary,
        })
    }
}
