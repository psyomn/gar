use rustc_serialize::json::Json;

/// This is one of the elements found within the GollumEvent payload
#[derive(Debug)]
struct PageElement {
    action: String,
    html_url: String,
    page_name: String,
    sha: String,
    summary: Option<String>,
}

impl PageElement {
    fn from_json(json: &Option<&Json>) -> Option<PageElement> {
        None
    }
}
