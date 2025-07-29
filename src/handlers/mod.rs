use std::sync::Arc;

use serde::Serialize;

use crate::{AppState, models::user::User};

pub mod auth;
pub mod homepage;

#[derive(Serialize, Clone)]
pub struct SharedTemplateContext {
    pub current_user: Option<User>,
    pub app_display_name: String,
    pub app_version: String,
}

impl SharedTemplateContext {
    pub fn new(app_state: &Arc<AppState>, current_user: Option<User>) -> Self {
        SharedTemplateContext {
            current_user,
            app_display_name: app_state.app_details.display_name.to_string(),
            app_version: app_state.app_details.version.to_string(),
        }
    }
}
