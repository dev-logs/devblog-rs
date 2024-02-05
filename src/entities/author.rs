#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub avatar_url: String
}

impl Author {
    pub fn new(name: &str, avatar_url: &str) -> Self {
        Self {
            name: name.to_string(),
            avatar_url: avatar_url.to_string()
        }
    }
}
