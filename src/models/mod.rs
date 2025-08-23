use async_trait::async_trait;

pub mod user;

#[async_trait]
pub trait Entity: Sized + Send + Sync {
    const TABLE: &'static str;
}
