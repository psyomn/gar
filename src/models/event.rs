use std::collections::BTreeMap;
use std::path::PathBuf;

use regex::Regex;
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
    created_at: Option<DateTime<Utc>>,
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

    pub fn set_owner_gh_id(&mut self, id: u64) -> () {
        self.owner.set_gh_id(id);
    }

    pub fn set_owner_nick(&mut self, n: String) -> () {
        self.owner.set_nick(n);
    }

    pub fn set_owner_email(&mut self, e: String) -> () {
        self.owner.set_email(e);
    }

    pub fn set_created_at(&mut self, e: DateTime<Utc>) -> () {
        self.created_at = Some(e);
    }

    /// Provided a list of constraints, this will return true only if ALL of them are satisfied
    pub fn satisfies_constraints(&self, v: &Vec<Constraint>) -> bool {
        let mut b = true;
        for cons in v.iter() {
            if cons.label == "language" {
                /* Example: language:Rust */
                b &= cons.value == self.language;
            }
            if cons.label == "owner" {
                /* Example: owner:psyomn */
                b &= cons.value == *self.owner.get_nick();
            }
            if cons.label == "name" {
                /* Example: name:wayland
                 * This will perform a regex match against the name of the repo
                 */
                let re_str: String = format!("(?i){}", cons.value);
                let re: Regex = Regex::new(re_str.as_ref()).unwrap();
                b &= re.is_match(self.name.as_ref());
            }
            if cons.label == "description" {
                /* This does a wor dmatch against the description given to the event's repo */
                let re_str: String = format!("(?i){}", cons.value);
                let re: Regex = Regex::new(re_str.as_ref()).unwrap();
                b &= re.is_match(self.description.as_ref());
            }
            if cons.label == "+watchers" {
                /* TODO: parsing to int each time - this might not be good? */
                let num: u64 = cons.value.parse::<u64>().ok().unwrap();
                b &= num <= self.watchers;
            }
            if cons.label == "-watchers" {
                /* TODO: parsing to int each time - this might not be good? */
                let num: u64 = cons.value.parse::<u64>().ok().unwrap();
                b &= num > self.watchers;
            }
            if cons.label == "+stargazers" {
                /* TODO: parsing to int each time - this might not be good? */
                let num: u64 = cons.value.parse::<u64>().ok().unwrap();
                b &= num <= self.stargazers;
            }
            if cons.label == "-stargazers" {
                /* TODO: parsing to int each time - this might not be good? */
                let num: u64 = cons.value.parse::<u64>().ok().unwrap();
                b &= num < self.stargazers;
            }
            if cons.label == "type" {
                let etype: &EventType = match self.event_type {
                    Some(ref v) => v,
                    None => continue,
                };

                b &= match cons.value.as_ref() {
                    "create"                      => match etype { &EventType::Create                   => true, _ => false },
                    "commit_comment"              => match etype { &EventType::CommitComment            => true, _ => false },
                    "delete"                      => match etype { &EventType::Delete(..)               => true, _ => false },
                    "deployment"                  => match etype { &EventType::Deployment               => true, _ => false },
                    "deployment_status"           => match etype { &EventType::DeploymentStatus         => true, _ => false },
                    "download"                    => match etype { &EventType::Download                 => true, _ => false },
                    "follow"                      => match etype { &EventType::Follow                   => true, _ => false },
                    "fork"                        => match etype { &EventType::Fork                     => true, _ => false },
                    "fork_apply"                  => match etype { &EventType::ForkApply                => true, _ => false },
                    "gist"                        => match etype { &EventType::Gist                     => true, _ => false },
                    "gollum"                      => match etype { &EventType::Gollum(..)               => true, _ => false },
                    "issue_comment"               => match etype { &EventType::IssueComment(..)         => true, _ => false },
                    "issues"                      => match etype { &EventType::Issues(..)               => true, _ => false },
                    "member"                      => match etype { &EventType::Member                   => true, _ => false },
                    "membership"                  => match etype { &EventType::Membership               => true, _ => false },
                    "page_build"                  => match etype { &EventType::PageBuild                => true, _ => false },
                    "public"                      => match etype { &EventType::Public                   => true, _ => false },
                    "pull_request"                => match etype { &EventType::PullRequest              => true, _ => false },
                    "pull_request_review_comment" => match etype { &EventType::PullRequestReviewComment => true, _ => false },
                    "push"                        => match etype { &EventType::Push(..)                 => true, _ => false },
                    "release"                     => match etype { &EventType::Release                  => true, _ => false },
                    "repository"                  => match etype { &EventType::Repository               => true, _ => false },
                    "status"                      => match etype { &EventType::Status                   => true, _ => false },
                    "team_add"                    => match etype { &EventType::TeamAdd                  => true, _ => false },
                    "watch"                       => match etype { &EventType::Watch(..)                => true, _ => false },
                    _                             => true, /* Ignore erroneous input */
                }
            }
            if cons.label == "commit_comment" {
                /* Prereq: for this match to happen, we want to make sure that we ahve a Push
                 * Event, as the match depends on the respective payload */

                let etype: &EventType = match self.event_type {
                    Some(ref v) => v,
                    None => continue,
                };

                match etype {
                    &EventType::Push(ref payload) => {
                        if let Some(ref payload) = *payload {
                            /* Does the commit comment contain some particular text? */
                            let txt: &str = cons.value.as_ref();
                            b &= payload.sha_elements_contain_text_of(txt);
                        }
                    },
                    _ => continue,
                }

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
            let json_line: Json = match Json::from_str(line.as_ref()) {
                Ok(v)  => v,
                Err(e) => {
                    // TODO cleanup
                    println!("Could not parse anything given:\n{}", line);
                    println!("Err: {}", e);

                    continue;
                },
            };

            if let Some(v) = Event::from_json(Some(&json_line)) {
                res.push(v);
            };
        }

        res
    }

    /// Given a json string, try to evaluate it into a repo
    pub fn from_json(json: Option<&Json>) -> Option<Event> {
        if json.is_none() { return None }
        if !json.unwrap().is_object() { return None }

        let obj = json.unwrap().as_object().unwrap();

        let created_at: Option<DateTime<Utc>> = match obj.get("created_at") {
            Some(v) => {
                match *v {
                    Json::String(ref s) => {
                        let date_str: String = s.clone();
                        match date_str.parse::<DateTime<Utc>>() {
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
                        "DeleteEvent"                   => Some(EventType::Delete(DeletePayload::from_json(obj.get("payload")))),
                        "DeploymentEvent"               => Some(EventType::Deployment),
                        "DeploymentStatusEvent"         => Some(EventType::DeploymentStatus),
                        "DownloadEvent"                 => Some(EventType::Download),
                        "FollowEvent"                   => Some(EventType::Follow),
                        "ForkEvent"                     => Some(EventType::Fork),
                        "ForkApplyEvent"                => Some(EventType::ForkApply),
                        "GistEvent"                     => Some(EventType::Gist),
                        "GollumEvent"                   => Some(EventType::Gollum(GollumPayload::from_json(obj.get("payload")))),
                        "IssueCommentEvent"             => Some(EventType::IssueComment(IssueCommentPayload::from_json(obj.get("payload")))),
                        "IssuesEvent"                   => Some(EventType::Issues(IssuePayload::from_json(obj.get("payload")))),
                        "MemberEvent"                   => Some(EventType::Member),
                        "MembershipEvent"               => Some(EventType::Membership),
                        "PageBuildEvent"                => Some(EventType::PageBuild),
                        "PublicEvent"                   => Some(EventType::Public),
                        "PullRequestEvent"              => Some(EventType::PullRequest),
                        "PullRequestReviewCommentEvent" => Some(EventType::PullRequestReviewComment),
                        "PushEvent"                     => Some(EventType::Push(PushPayload::from_json(obj.get("payload")))),
                        "ReleaseEvent"                  => Some(EventType::Release),
                        "RepositoryEvent"               => Some(EventType::Repository),
                        "StatusEvent"                   => Some(EventType::Status),
                        "TeamAddEvent"                  => Some(EventType::TeamAdd),
                        "WatchEvent"                    => Some(EventType::Watch(WatchPayload::from_json(obj.get("payload")))),
                        _                               => None,
                    }
                },
                _ => None,
            },
        };

        let gh_id = JsonHelper::number_or_zero(repo.get("id"));
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

        repo.set_owner_nick(owner_name);

        repo.gh_id = gh_id;
        repo.url = url;
        repo.description = desc;
        repo.name = name;
        repo.has_issues = issues_present;
        repo.language = language;
        repo.event_type = event;
        repo.stargazers = num_stargazers;
        repo.watchers = watchers;
        repo.forks = forks;
        repo.created_at = created_at;
        repo.open_issues = open_issues;

        Some(repo)
    }

    /// Gives a flat json hash with labels and values.
    pub fn to_btree_with_features_of(&self, f: Vec<String>) -> BTreeMap<String, String> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();

        let id_label: String = "id".into();
        let name_label: String = "name".into();
        let desc: String = "description".into();
        let lang: String = "language".into();
        let has_issues: String = "has_issues".into();
        let owner: String = "owner".into();
        let url: String = "url".into();
        let watchers: String = "watchers".into();
        let stargazers: String = "stargazers".into();
        let forks: String = "forks".into();
        let open_issues: String = "forks".into();
        let event_type: String = "event_type".into();
        let created_at: String = "created_at".into();

        if ::vec_contains(&f, &id_label) {
            map.insert(id_label, self.gh_id.to_string());
        }
        if ::vec_contains(&f, &name_label) {
            map.insert(name_label, self.name.clone());
        }
        if ::vec_contains(&f, &desc) {
            map.insert(desc, self.description.clone());
        }
        if ::vec_contains(&f, &lang) {
            map.insert(lang, self.language.clone());
        }
        if ::vec_contains(&f, &has_issues) {
            map.insert(has_issues, self.has_issues.to_string());
        }
        if ::vec_contains(&f, &owner) {
            map.insert(owner, self.owner.get_nick().clone());
        }
        if ::vec_contains(&f, &url) {
            map.insert(url, self.url.clone());
        }
        if ::vec_contains(&f, &watchers) {
            map.insert(watchers, self.watchers.to_string());
        }
        if ::vec_contains(&f, &stargazers) {
            map.insert(stargazers, self.stargazers.to_string());
        }
        if ::vec_contains(&f, &forks) {
            map.insert(forks, self.forks.to_string());
        }
        if ::vec_contains(&f, &open_issues) {
            map.insert(open_issues, self.open_issues.to_string());
        }
        if ::vec_contains(&f, &event_type) {
            if let Some(ref etype) = self.event_type {
                map.insert(event_type, etype.to_string());
            }
            else {
                map.insert(event_type, "null".into());
            }
        }
        if ::vec_contains(&f, &created_at) {
            if let Some(date) = self.created_at {
                map.insert(created_at, date.to_rfc3339());
            }
            else {
                map.insert(created_at, "null".into());
            }
        }

        map
    }

    pub fn to_btree_with_all_features(&self) -> BTreeMap<String, String> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();

        map.insert("id".into(), self.gh_id.to_string());
        map.insert("name".into(), self.name.clone());
        map.insert("description".into(), self.description.clone());
        map.insert("language".into(), self.language.clone());
        map.insert("has_issues".into(), self.has_issues.to_string());
        map.insert("owner".into(), self.owner.get_nick().clone());
        map.insert("url".into(), self.url.clone());
        map.insert("watchers".into(), self.watchers.to_string());
        map.insert("stargazers".into(), self.stargazers.to_string());
        map.insert("forks".into(), self.forks.to_string());

        if let Some(ref etype) = self.event_type {
            map.insert("event_type".into(), etype.to_string());
        }
        if let Some(date) = self.created_at {
            map.insert("created_at".into(), date.to_rfc3339());
        }

        map
    }
}
