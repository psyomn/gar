use rustc_serialize::json::*;

#[derive(Debug)]
pub struct IssuePayload {
    action: String,
    issue: u64,
    number: u64,
}

impl IssuePayload {
    pub fn from_json(json: &Option<&Json>) -> Option<IssuePayload> {
        None
    }
}

type IssuePayloadBuilder = IssuePayload;

impl IssuePayloadBuilder {
    pub fn new() -> IssuePayloadBuilder {
        IssuePayloadBuilder {
            action: "".into(),
            issue: 0,
            number: 0,
        }
    }

    pub fn action(self, s: String) -> IssuePayloadBuilder {
        IssuePayloadBuilder {
            action: s,
            issue: self.issue,
            number: self.number,
        }
    }

    pub fn issue(self, i: u64) -> IssuePayloadBuilder {
        IssuePayloadBuilder {
            action: self.action,
            issue: i,
            number: self.number,
        }
    }

    pub fn number(self, n: u64) -> IssuePayloadBuilder {
        IssuePayloadBuilder {
            action: self.action,
            issue: self.issue,
            number: self.number,
        }
    }

    pub fn finalize(self) -> IssuePayload {
        IssuePayload {
            action: self.action,
            issue: self.issue,
            number: self.number,
        }
    }
}
