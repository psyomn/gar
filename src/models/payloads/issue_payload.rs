use rustc_serialize::json::*;

#[derive(Debug)]
pub struct IssuePayload {
    action: String,
    issue: u64,
    number: u64,
}

impl IssuePayload {
    pub fn from_json(json: Option<&Json>) -> Option<IssuePayload> {
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

pub struct IssuePayloadBuilder {
    action: String,
    issue: u64,
    number: u64,
}

impl IssuePayloadBuilder {
    pub fn new() -> IssuePayloadBuilder {
        IssuePayloadBuilder {
            action: "".into(),
            issue: 0,
            number: 0,
        }
    }

    pub fn action(&mut self, s: String) -> &mut IssuePayloadBuilder {
        self.action = s;
        self
    }

    pub fn issue(&mut self, i: u64) -> &mut IssuePayloadBuilder {
        self.issue = i;
        self
    }

    pub fn number(&mut self, n: u64) -> &mut IssuePayloadBuilder {
        self.number = n;
        self
    }

    pub fn finalize(&self) -> IssuePayload {
        IssuePayload {
            action: self.action.clone(),
            issue: self.issue,
            number: self.number,
        }
    }
}
