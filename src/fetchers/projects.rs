use chrono::{DateTime, Local};
use leptos::{prelude::ServerFnError, *};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::helpers::projects::{ProjectInfo, ProjectsResponse};

// Resource to manage projects data
// #[server(GetProjects)]
pub async fn get_projects() -> Vec<ProjectInfo> {
    let client = Client::new();
    
    // Replace with your actual API endpoint
    let projects_response = client
        .get("http://localhost:3000/api/projects")
        .send()
        .await.expect("Couldn't make projects call")
        .json::<ProjectsResponse>()
        .await.expect("Couldn't make projects call");

    // Transform the API response into ProjectInfo objects
    let mut projects: Vec<ProjectInfo> = projects_response
        .projects
        .into_iter()
        .map(|data| ProjectInfo {
            project_id: data.project_id,
            project_name: data.project_name,
            dir_name: "".to_string(),
            // Assuming the API now provides these timestamps
            created: Local::now(), // Replace with actual created from API
            modified: Local::now(), // Replace with actual modified from API
        })
        .collect();

    // Sort by modification date (newest first)
    projects.sort_by(|a, b| b.modified.cmp(&a.modified));

    projects
}

