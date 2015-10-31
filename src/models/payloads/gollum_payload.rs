use rustc_serialize::json::Json;
use models::payloads::page_element::PageElement;

pub struct GollumPayload {
    pages: Vec<PageElement>,
}
