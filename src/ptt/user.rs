#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: String,
    name: Option<String>,
}

impl User {
    pub fn new(id: String, name: Option<String>) -> Self {
        User { id, name }
    }
}
