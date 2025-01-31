use chrono::{DateTime, Local};
use gloo_net::http::Request;
use leptos::{prelude::ServerFnError, *};
use serde::{Deserialize, Serialize};

use crate::helpers::projects::{ProjectInfo, ProjectsResponse};

pub async fn get_projects(token: String) -> Vec<ProjectInfo> {
    // Send the POST request using `gloo-net`
    let response = Request::post("http://localhost:3000/api/projects/all")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        // .json(&login_request)
        // .expect("Failed to serialize login request")
        .send()
        .await
        .expect("Failed to send login request");

    // Check if the response is successful
    if response.ok() {
        // Parse the JSON response
        let projects_response: ProjectsResponse = response
            .json()
            .await
            .expect("Failed to parse login response");

        // Transform the API response into ProjectInfo objects
    let mut projects: Vec<ProjectInfo> = projects_response
        .projects
        .into_iter()
        .map(|data| ProjectInfo {
            project_id: data.project_id,
            project_name: data.project_name,
            dir_name: "".to_string(),
            created: DateTime::from(data.created_at.expect("Couldn't get datetime")),
            modified: DateTime::from(data.updated_at.expect("Couldn't get datetime")),
        })
        .collect();

    // Sort by modification date (newest first)
    projects.sort_by(|a, b| b.modified.cmp(&a.modified));

    projects
    } else {
        // Handle the error case
        panic!("Login failed: {}", response.status_text());
    }
}

