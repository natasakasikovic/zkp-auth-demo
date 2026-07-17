pub trait AuthProvider: Send + Sync {
    fn authorization_header(&self) -> String;

    fn verify_authorization_header(&self, header: Option<&str>) -> bool;
}
