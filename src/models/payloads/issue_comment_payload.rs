use rustc_serialize::json::Json;
use models::json_helpers::JsonHelper;

#[derive(Debug,Eq,PartialEq)]
pub struct IssueCommentPayload {
    comment_id: u64,
    issue_id: u64,
}

impl IssueCommentPayload {
    pub fn from_json(json: Option<&Json>) -> Option<IssueCommentPayload> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let obj = json.unwrap().as_object().unwrap();

        let cid: u64 = JsonHelper::number_or_zero(obj.get("comment_id"));
        let iid: u64 = JsonHelper::number_or_zero(obj.get("issue_id"));

        Some(IssueCommentPayload {
            comment_id: cid,
            issue_id: iid,
        })
    }
}
