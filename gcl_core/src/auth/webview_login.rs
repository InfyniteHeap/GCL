#[derive(Default)]
pub struct OAuth2Token {
    pub code: String,
}

impl OAuth2Token {
    pub fn new() -> Self {
        OAuth2Token {
            code: String::new(),
        }
    }

    pub fn login(&mut self) {
        todo!()
    }
}
