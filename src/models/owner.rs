#[derive(Debug)]
pub struct Owner {
    gh_id: u64,
    nick: String,
    email: String,
}

impl Owner {
    fn new() -> Owner {
        Owner {
            gh_id: 0,
            nick: "default".into(),
            email: "default@default".into(),
        }
    }

    pub fn set_gh_id(&mut self, id: u64) -> () {
        self.gh_id = id;
    }

    pub fn set_nick(&mut self, n: String) -> () {
        self.nick = n;
    }

    pub fn set_email(&mut self, e: String) -> () {
        self.email = e;
    }
}

pub struct OwnerBuilder {
    owner: Owner,
}

/// Builder for the owner object (notice that Owner#new is private)
impl OwnerBuilder {
    pub fn new() -> OwnerBuilder {
        OwnerBuilder {
            owner: Owner::new()
        }
    }

    pub fn gh_id(self, id: u64) -> OwnerBuilder {
        OwnerBuilder {
            owner: Owner {
                gh_id: id,
                nick: self.owner.nick,
                email: self.owner.email,
            }
        }
    }

    pub fn nick(self, nick: &str) -> OwnerBuilder {
        OwnerBuilder{
            owner: Owner {
                gh_id: self.owner.gh_id,
                nick: nick.into(),
                email: self.owner.email
            }
        }
    }

    pub fn email(self, email: &str) -> OwnerBuilder {
        OwnerBuilder {
            owner: Owner {
                gh_id: self.owner.gh_id,
                nick: self.owner.nick,
                email: email.into(),
            }
        }
    }

    pub fn finalize(self) -> Owner {
        self.owner
    }
}
