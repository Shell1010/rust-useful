
pub struct Set;
pub struct NotSet;

pub struct User {
    pub username: String,
    pub email: Option<String>,
}

pub struct UserBuilder<State> {
    username: Option<String>,
    email: Option<String>,
    _marker: State,
}

// Initial state: username is NotSet
impl UserBuilder<NotSet> {
    pub fn new() -> Self {
        Self {
            username: None,
            email: None,
            _marker: NotSet,
        }
    }

    pub fn username(self, username: impl Into<String>) -> UserBuilder<Set> {
        UserBuilder {
            username: Some(username.into()),
            email: self.email,
            _marker: Set,
        }
    }
}

// Intermediate state where username is Set
impl UserBuilder<Set> {
    // Optional email
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    // Build is only allowed when required fields are set
    pub fn build(self) -> User {
        User {
            username: self.username.unwrap(), // safe, guaranteed set
            email: self.email,
        }
    }
}
