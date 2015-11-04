use std::path::PathBuf;
use rustc_serialize::json::Json;

use models::owner;
use models::reader::lines_of;
use models::constraint::Constraint;
use models::event_type::EventType;
use models::json_helpers::JsonHelper;

use models::payloads::*;

use chrono::*;

#[derive(Debug)]
pub struct Event {
    gh_id: u64,
    name: String,
    description: String,
    language: String,
    has_issues: bool,
    owner: owner::Owner,
    url: String,
    watchers: u64,
    stargazers: u64,
    forks: u64,
    open_issues: u64,
    event_type: Option<EventType>,
    created_at: Option<DateTime<UTC>>,
}

/// Models a repo event, in the file obtained from githubarchive.
impl Event {
    pub fn new() -> Event {
        Event {
            gh_id: 0,
            name: "default".into(),
            language: "default".into(),
            description: "default".into(),
            has_issues: false,
            owner: owner::OwnerBuilder::new().finalize(),
            url: "default".into(),
            watchers: 0,
            stargazers: 0,
            forks: 0,
            open_issues: 0,
            event_type: None,
            created_at: None,
        }
    }

    pub fn set_watchers(&mut self, w: u64) -> () {
        self.watchers = w;
    }

    pub fn set_stargazers(&mut self, s: u64) -> () {
        self.stargazers = s;
    }

    pub fn set_forks(&mut self, f: u64) -> () {
        self.forks = f;
    }

    pub fn set_event_type(&mut self, e: Option<EventType>) -> () {
        self.event_type = e;
    }

    pub fn set_description(&mut self, s: String) -> () {
        self.description = s;
    }

    pub fn set_gh_id(&mut self, id: u64) -> () {
        self.gh_id = id;
    }

    pub fn set_name(&mut self, s: String) -> () {
        self.name = s;
    }

    pub fn set_language(&mut self, l: String) -> () {
        self.language = l;
    }

    pub fn set_has_issues(&mut self, b: bool) -> () {
        self.has_issues = b;
    }

    pub fn set_url(&mut self, s: String) -> () {
        self.url = s;
    }

    pub fn set_owner_gh_id(&mut self, id: u64) -> () {
        self.owner.set_gh_id(id);
    }

    pub fn set_owner_nick(&mut self, n: String) -> () {
        self.owner.set_nick(n);
    }

    pub fn set_owner_email(&mut self, e: String) -> () {
        self.owner.set_email(e);
    }

    pub fn set_created_at(&mut self, e: DateTime<UTC>) -> () {
        self.created_at = Some(e);
    }

    /// Provided a list of constraints, this will return true only if ALL of them are satisfied
    pub fn satisfies_constraints(&self, v: &Vec<Constraint>) -> bool {
        let mut b = true;
        for cons in v.iter() {
            if cons.label == "language" {
                b &= cons.value == self.language;
            }
            if cons.label == "owner" {
                b &= cons.value == *self.owner.get_nick();
            }
        }
        b
    }

    /// Given a path to a json.gz file, that file is read, and each line is parsed to a Event
    /// object.
    pub fn from_path(p: PathBuf) -> Vec<Event> {
        let v: Vec<String> = lines_of(p);
        let mut res: Vec<Event> = Vec::new();

        for line in v.into_iter() {
            let r: Event = match Event::from_json(line) {
                Some(v) => v,
                None => continue,
            };
            res.push(r);
        }

        res
    }

    /// Given a json string, try to evaluate it into a repo
    pub fn from_json(data: String) -> Option<Event> {
        let dt = match Json::from_str(data.as_ref()) {
            Ok(v) => v,
            Err(e) => {
                ::print_red(format!("Could not parse anything given:\n{}", data).as_ref());
                println!("Err: {}", e);
                return None;
            },
        };

        if !dt.is_object() { return None }

        let obj = dt.as_object().unwrap();

        let created_at: Option<DateTime<UTC>> = match obj.get("created_at") {
            Some(v) => {
                match *v {
                    Json::String(ref s) => {
                        let date_str: String = s.clone();
                        match date_str.parse::<DateTime<UTC>>() {
                            Ok(v) => Some(v),
                            Err(..) => None,
                        }
                    },
                    _ => None,
                }
            },
            None => None
        };

        let repo = match obj.get("repository") {
            None => return None,
            Some(v) => v.as_object().unwrap(),
        };

        let event: Option<EventType> = match obj.get("type") {
            None => None,
            Some(v) => match *v {
                Json::String(ref s) => {
                    match s.as_ref() {
                        "CreateEvent"                   => Some(EventType::Create),
                        "CommitCommentEvent"            => Some(EventType::CommitComment),
                        "DeleteEvent"                   => Some(EventType::Delete(DeletePayload::from_json(&obj.get("payload")))),
                        "DeploymentEvent"               => Some(EventType::Deployment),
                        "DeploymentStatusEvent"         => Some(EventType::DeploymentStatus),
                        "DownloadEvent"                 => Some(EventType::Download),
                        "FollowEvent"                   => Some(EventType::Follow),
                        "ForkEvent"                     => Some(EventType::Fork),
                        "ForkApplyEvent"                => Some(EventType::ForkApply),
                        "GistEvent"                     => Some(EventType::Gist),
                        "GollumEvent"                   => Some(EventType::Gollum(GollumPayload::from_json(&obj.get("payload")))),
                        "IssueCommentEvent"             => Some(EventType::IssueComment(IssueCommentPayload::from_json(&obj.get("payload")))),
                        "IssuesEvent"                   => Some(EventType::Issues(IssuePayload::from_json(&obj.get("payload")))),
                        "MemberEvent"                   => Some(EventType::Member),
                        "MembershipEvent"               => Some(EventType::Membership),
                        "PageBuildEvent"                => Some(EventType::PageBuild),
                        "PublicEvent"                   => Some(EventType::Public),
                        "PullRequestEvent"              => Some(EventType::PullRequest),
                        "PullRequestReviewCommentEvent" => Some(EventType::PullRequestReviewComment),
                        "PushEvent"                     => Some(EventType::Push(PushPayload::from_json(&obj.get("payload")))),
                        "ReleaseEvent"                  => Some(EventType::Release),
                        "RepositoryEvent"               => Some(EventType::Repository),
                        "StatusEvent"                   => Some(EventType::Status),
                        "TeamAddEvent"                  => Some(EventType::TeamAdd),
                        "WatchEvent"                    => Some(EventType::Watch(WatchPayload::from_json(&obj.get("payload")))),
                        _                               => None,
                    }
                },
                _ => None,
            },
        };

        let gh_id = match repo.get("id") {
            Some(v) => match *v {
                Json::U64(id) => id,
                _ => 0,
            },
            None => 0
        };

        let name: String = JsonHelper::string_or_empty(repo.get("name"));
        let url: String = JsonHelper::string_or_empty(repo.get("url"));
        let desc: String = JsonHelper::string_or_empty(repo.get("description"));
        let owner_name: String = JsonHelper::string_or_empty(repo.get("owner"));
        let issues_present: bool = JsonHelper::boolean_or_false(repo.get("has_issues"));
        let language: String = JsonHelper::string_or_empty(repo.get("language"));
        let num_stargazers: u64 = JsonHelper::number_or_zero(repo.get("stargazers"));
        let watchers: u64 = JsonHelper::number_or_zero(repo.get("watchers"));
        let forks: u64 = JsonHelper::number_or_zero(repo.get("forks"));
        let open_issues: u64 = JsonHelper::number_or_zero(repo.get("open_issues"));

        let mut repo: Event = Event::new();

        repo.set_gh_id(gh_id);
        repo.set_url(url);
        repo.set_owner_nick(owner_name);
        repo.set_description(desc);
        repo.set_name(name);
        repo.set_has_issues(issues_present);
        repo.set_language(language);
        repo.set_event_type(event);
        repo.set_stargazers(num_stargazers);
        repo.watchers = watchers;
        repo.forks = forks;
        repo.created_at = created_at;
        repo.open_issues = open_issues;

        Some(repo)
    }
}

#[cfg(test)]
mod test {
    use models::repo::Event;
    #[test]
    fn test_json_parse_simple() -> () {
        let r: Option<Event> = Event::from_json("{\"name\":\"potato\"".into());
        assert!(r.is_none());
    }
}