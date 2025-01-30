use chrono::{DateTime, Local};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProjectInfo {
    pub dir_name: String,
    pub project_id: String,
    pub project_name: String,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
}
