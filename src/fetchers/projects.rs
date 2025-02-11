use chrono::{DateTime, Local};
use gloo_net::http::Request;
use leptos::{prelude::ServerFnError, *};
use serde::{Deserialize, Serialize};

use crate::helpers::{projects::{CreateProjectRequest, CreateProjectResponse, ProjectInfo, ProjectsResponse}, utilities::SavedState};

pub async fn get_projects(token: String) -> Vec<ProjectInfo> {
    // Send the POST request using `gloo-net`
    let response = Request::get("http://localhost:3000/api/projects/all")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
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
            project_id: data.id,
            project_name: data.name,
            // dir_name: "".to_string(),
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

pub async fn create_project(token: String, name: String, empty_file_data: SavedState) -> CreateProjectResponse {
    let create_request = CreateProjectRequest { name, empty_file_data };

    let response = Request::post("http://localhost:3000/api/projects/create")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&create_request)
        .expect("Failed to serialize project request")
        .send()
        .await
        .expect("Failed to send project request");

    // Check if the response is successful
    if response.ok() {
        // Parse the JSON response
        let project_response: CreateProjectResponse = response
            .json()
            .await
            .expect("Failed to parse project response");

        project_response
    } else {
        // Handle the error case
        panic!("Project failed: {}", response.status_text());
    }
}

