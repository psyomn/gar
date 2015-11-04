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

    pub fn get_nick(&self) -> &String {
        &self.email
    }
}

pub struct OwnerBuilder {
    gh_id: u64,
    nick: String,
    email: String,
}

/// Builder for the owner object (notice that Owner#new is private)
impl OwnerBuilder {
    pub fn new() -> OwnerBuilder {
        OwnerBuilder {
            gh_id: 0,
            nick: "".into(),
            email: "".into(),
        }
    }

    pub fn gh_id(&mut self, id: u64) -> &mut OwnerBuilder {
        self.gh_id = id;
        self
    }

    pub fn nick(&mut self, nick: &str) -> &mut OwnerBuilder {
        self.nick = nick.into();
        self
    }

    pub fn email(&mut self, email: &str) -> &mut OwnerBuilder {
        self.email = email.into();
        self
    }

    pub fn finalize(&self) -> Owner {
        Owner {
            gh_id: self.gh_id,
            nick: self.nick.clone(),
            email: self.email.clone(),
        }
    }
}
