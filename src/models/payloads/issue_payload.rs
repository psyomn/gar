use rustc_serialize::json::*;

#[derive(Debug)]
pub struct IssuePayload {
    action: String,
    issue: u64,
    number: u64,
}

impl IssuePayload {
    pub fn from_json(json: &Option<&Json>) -> Option<IssuePayload> {
        if json.is_none() { return None }

        let ref obj = *json.unwrap();

        if !obj.is_object() { return None }

        let json = obj.as_object().unwrap();

        let action: String = match json.get("action") {
            Some(v) => match *v {
                Json::String(ref s) => s.clone(),
                _ => "".into(),
            },
            None => "".into(),
        };

        let issue: u64 = match json.get("issue") {
            Some(v) => match *v {
                Json::U64(v) => v,
                _ => 0,
            },
            None => 0,
        };

        let number: u64 = match json.get("number") {
            Some(v) => match *v {
                Json::U64(v) => v,
                _ => 0,
            },
            None => 0,
        };

        Some(
            IssuePayloadBuilder::new()
                .action(action)
                .issue(issue)
                .number(number)
                .finalize())
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
            number: n,
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
