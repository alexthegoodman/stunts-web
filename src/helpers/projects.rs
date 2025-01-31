use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProjectInfo {
    pub dir_name: String,
    pub project_id: String,
    pub project_name: String,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub project_id: String,
    pub project_name: String,
    #[serde(with = "ts_seconds_option")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub created_at: Option<DateTime<Utc>>,
}

// API response type
#[derive(Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<ProjectData>,
}
