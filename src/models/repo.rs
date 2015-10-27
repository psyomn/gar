use rustc_serialize::json::Json;
use models::owner;

#[derive(Debug)]
pub struct Repo {
    gh_id: u64,
    name: String,
    language: String,
    has_issues: bool,
    owner: owner::Owner,
    url: String,
}

/// Models a repo event, in the file obtained from githubarchive.
impl Repo {
    pub fn new() -> Repo {
        Repo {
            gh_id: 0,
            name: "default".into(),
            language: "default".into(),
            has_issues: false,
            owner: owner::OwnerBuilder::new().finalize(),
            url: "default".into(),
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

    /// Given a json string, try to evaluate it into a repo
    pub fn from_json(data: String) -> Option<Repo> {
        let dt = match Json::from_str(data.as_ref()) {
            Ok(v) => v,
            Err(e) => {
                println!("Could not parse anything given:\n{}", data);
                println!("Err: {}", e);
                return None;
            },
        };

        println!("{:?}", dt);

        if !dt.is_object() { return None }

        let rb: RepoBuilder = RepoBuilder::new();
        let obj = dt.as_object().unwrap_or(return None);

        let repo = match obj.get("repository") {
            None => return None,
            Some(v) => v.as_object().unwrap(),
        };

        let gh_id = match repo.get("id") {
            Some(v) => match *v {
                Json::U64(id) => id,
                _ => 0,
            },
            _ => 0
        };
        println!("read id: {}", gh_id);

        Some(Repo::new())
    }
}

pub struct RepoBuilder {
    repo: Repo,
}

impl RepoBuilder {
    pub fn new() -> RepoBuilder {
        RepoBuilder {
            repo: Repo::new(),
        }
    }
}


mod test {
    use super::*;
    #[test]
    fn test_json_parse_simple() -> () {
        let r: Option<Repo> = Repo::from_json("{\"name\":\"potato\"".into());
        match r {
            None => assert!(true),
            _ => assert!(false),
        }
    }
}
