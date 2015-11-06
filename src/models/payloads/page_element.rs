use rustc_serialize::json::Json;
use models::json_helpers::JsonHelper;

/// This is one of the elements found within the GollumEvent payload
#[derive(Debug)]
pub struct PageElement {
    action: String,
    html_url: String,
    page_name: String,
    sha: String,
    summary: Option<String>,
}

impl PageElement {
    pub fn from_json(json: &Option<&Json>) -> Option<PageElement> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let obj = json.unwrap().as_object().unwrap();

        let action    = JsonHelper::string_or_empty(obj.get("action"));
        let html_url  = JsonHelper::string_or_empty(obj.get("html_url"));
        let page_name = JsonHelper::string_or_empty(obj.get("page_name"));
        let sha       = JsonHelper::string_or_empty(obj.get("sha"));

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
