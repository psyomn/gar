use std::io::ErrorKind;
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

// TODO: this should have an enumerated struct, because not all the
//   events have the same fields.
#[derive(Debug)]
pub struct Event {
    gh_id: u64,
    name: String,
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

impl Default for Event {
    fn default() -> Self {
        Event{
            gh_id: 0,
            name: "".into(),
            language: "".into(),
            has_issues: false,
            owner: owner::OwnerBuilder::new().finalize(),
            url: "".into(),
            watchers: 0,
            stargazers: 0,
            forks: 0,
            open_issues: 0,
            event_type: None,
            created_at: None,
        }
    }
}

/// Models a repo event, in the file obtained from githubarchive.
impl Event {
    pub fn new() -> Event { Default::default() }

    pub fn get_gh_id(&self) -> u64 { self.gh_id }
    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_language(&self) -> &String { &self.language }

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
            // TODO: maybe make the payload description searchable
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

                // TODO: there should bea better way to do this --
                // probably can convert the constraint into the enum
                // and make the comparison like this
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

    /// Given a path to a json.gz file, that file is read, and each
    /// line is parsed to a Event object.
    pub fn from_path(p: PathBuf) -> Vec<Event> {
        let v: Vec<String> = lines_of(p);
        let mut res: Vec<Event> = Vec::new();


        for line in v.into_iter() {
            let obj = match Event::into_json_object(&line) {
                Ok(v) => v,
                Err(..) => continue, // TODO: logging should be good here
            };

            res.push(obj);
        }

        res
    }

    pub fn into_json_object(line: &str) -> std::io::Result<Event> {
        // cleanup stdio::Error

        let json_line: Json = match Json::from_str(line) {
            Ok(v)  => v,
            Err(e) => {
                // TODO cleanup
                println!("Could not parse anything given:\n{}", line);
                println!("Err: {}", e);
                return Err(std::io::Error::new(ErrorKind::InvalidInput, "invalid json"))
            },
        };

        match Event::from_json(&json_line) {
            Some(v) => Ok(v),
            None => {
                return Err(std::io::Error::new(ErrorKind::InvalidInput, "bad json structure"))
            },
        }
    }

    /// Given a json string, try to evaluate it into a repo
    pub fn from_json(json: &Json) -> Option<Event> {
        const KEY_CREATED_AT: &str = "created_at";
        const KEY_REPOSITORY: &str = "repo";
        const KEY_TYPE: &str = "type";
        // I'm adding the above as constants; some of the keys changed
        // in the json over the years, and we should have one source of
        // truth.

        if !json.is_object() { return None }

        let obj = json.as_object().unwrap();

        let created_at: Option<DateTime<Utc>> = match obj.get(KEY_CREATED_AT) {
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

        let payload_obj = obj.get("payload");

        let event: Option<EventType> = match obj.get(KEY_TYPE) {
            None => None,
            Some(v) => match *v {
                Json::String(ref s) => {
                    match s.parse::<EventType>().unwrap() {
                        EventType::Unknown(..) => None, // preserve Unknown in the future
                        EventType::Delete(..) => Some(EventType::Delete(DeletePayload::from_json(payload_obj))),
                        EventType::Gollum(..) => Some(EventType::Gollum(GollumPayload::from_json(payload_obj))),
                        EventType::IssueComment(..) => Some(EventType::IssueComment(IssueCommentPayload::from_json(payload_obj))),
                        EventType::Issues(..) => Some(EventType::Issues(IssuePayload::from_json(payload_obj))),
                        EventType::Push(..) => Some(EventType::Push(PushPayload::from_json(payload_obj))),
                        EventType::Watch(..) => Some(EventType::Watch(WatchPayload::from_json(payload_obj))),
                        simple_event => Some(simple_event),
                    }
                },
                _ => None,
            },
        };

        let repo = match obj.get(KEY_REPOSITORY) {
            None => return None,
            Some(v) => v.as_object().unwrap(),
        };

        let gh_id = JsonHelper::number_or_zero(repo.get("id"));
        let name: String = JsonHelper::string_or_empty(repo.get("name"));
        let url: String = JsonHelper::string_or_empty(repo.get("url"));
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
