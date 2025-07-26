use crate::models::ghub_app::GHUBApp;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub applications: Mutex<Vec<GHUBApp>>,
    pub settings_db_data: Mutex<Option<ApplicationsData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationsData {
    pub applications: Vec<GHUBApp>,
}