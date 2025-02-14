use chrono::serde::ts_seconds_option;
use chrono::{DateTime, FixedOffset, Local, Utc};
use serde::{Deserialize, Serialize};
use stunts_engine::animations::Sequence;
use stunts_engine::timelines::SavedTimelineStateConfig;

use super::utilities::SavedState;

#[derive(PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct StoredProject {
    pub project_id: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ProjectInfo {
    // pub dir_name: String,
    pub project_id: String,
    pub project_name: String,
    pub created: DateTime<Local>,
    pub modified: DateTime<Local>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub id: String,
    pub name: String,
    // #[serde(with = "ts_seconds_option")]
    // pub updated_at: Option<DateTime<Utc>>,
    // #[serde(with = "ts_seconds_option")]
    // pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SingleProjectData {
    pub id: String,
    pub name: String,
    pub file_data: SavedState,
    pub updated_at: Option<DateTime<FixedOffset>>,
    pub created_at: Option<DateTime<FixedOffset>>,
}

#[derive(Deserialize)]
pub struct SingleProjectResponse {
    pub project: SingleProjectData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleProjectRequest {
    pub project_id: String,
}

// API response type
#[derive(Deserialize)]
pub struct ProjectsResponse {
    pub projects: Vec<ProjectData>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub name: String,
    pub empty_file_data: SavedState,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectResponse {
    pub new_project: ProjectData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSequencesRequest {
    pub project_id: String,
    pub sequences: Vec<Sequence>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSequencesResponse {
    pub updated_project: ProjectData,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimelineRequest {
    pub project_id: String,
    pub timeline_state: SavedTimelineStateConfig,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTimelineResponse {
    pub updated_project: ProjectData,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadResponse {
    pub url: String,
    pub file_name: String,
    pub size: u32,
    pub mime_type: String,
}
