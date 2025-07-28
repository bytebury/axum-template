pub mod google;

pub trait OAuth {
    fn auth_url(&self) -> String;
    fn exchange_code_for_access_token(&self, code: &str) -> impl Future<Output = String>;
}
