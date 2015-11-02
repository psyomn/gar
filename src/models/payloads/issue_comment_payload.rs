#[derive(Debug)]
pub struct IssueCommentPayload {
    comment_id: u64,
    issue_id: u64,
}

impl IssueCommentPayload {
    fn new(c: u64, i: u64) -> IssueCommentPayload {
        IssueCommentPayload {
            comment_id: c,
            issue_id: i,
        }
    }
}
