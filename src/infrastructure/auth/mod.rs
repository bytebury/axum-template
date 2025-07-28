use crate::models::user::User;

pub mod google;

pub trait OAuth {
    fn auth_url(&self) -> String;
    fn exchange_code_for_user(&self, code: &str) -> impl Future<Output = Option<User>>;
}
