use super::AuthProvider;

#[derive(Debug, Clone)]
pub struct DemoTokenAuthProvider {
    token: String,
}

impl DemoTokenAuthProvider {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }
}

impl AuthProvider for DemoTokenAuthProvider {
    fn authorization_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    fn verify_authorization_header(&self, header: Option<&str>) -> bool {
        header
            .map(|value| value == self.authorization_header())
            .unwrap_or(false)
    }
}
