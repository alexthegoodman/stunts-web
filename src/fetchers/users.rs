use std::time::Duration;

use chrono::{DateTime, Local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

use crate::helpers::users::{LoginRequest, LoginResponse, SubscriptionDetails};

// pub fn set_authenticated(
//     auth_state: RwSignal<AuthState>,
//     token: String,
//     expiry: Option<chrono::DateTime<chrono::Utc>>,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let auth_token = AuthToken { token, expiry };
//     save_auth_token(&auth_token)?;

//     auth_state.set(AuthState {
//         token: Some(auth_token),
//         is_authenticated: true,
//         subscription: None, // Will be updated by the effect
//     });

//     Ok(())
// }
// // Function to handle logout
// pub fn logout(auth_state: RwSignal<AuthState>) -> Result<(), Box<dyn std::error::Error>> {
//     clear_auth_token()?;

//     auth_state.set(AuthState {
//         token: None,
//         is_authenticated: false,
//         subscription: None,
//     });

//     Ok(())
// }

pub async fn login_user(email: String, password: String) -> LoginResponse {
    // Create the JSON body for the request
    let login_request = LoginRequest { email, password };

    // Send the POST request using `gloo-net`
    let response = Request::post("http://localhost:3000/api/auth/login")
        .header("Content-Type", "application/json")
        .json(&login_request)
        .expect("Failed to serialize login request")
        .send()
        .await
        .expect("Failed to send login request");

    // Check if the response is successful
    if response.ok() {
        // Parse the JSON response
        let login_response: LoginResponse = response
            .json()
            .await
            .expect("Failed to parse login response");
        login_response
    } else {
        // Handle the error case
        panic!("Login failed: {}", response.status_text());
    }
}

pub async fn fetch_subscription_details(token: &str,) -> SubscriptionDetails {
    // Send the POST request using `gloo-net`
    let response = Request::post("http://localhost:3000/api/subscription/details")
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        // .expect("Failed to serialize details request")
        .send()
        .await
        .expect("Failed to send details request");

    // Check if the response is successful
    if response.ok() {
        // Parse the JSON response
        let details: SubscriptionDetails = response
            .json()
            .await
            .expect("Failed to parse details response");
        details
    } else {
        // Handle the error case
        panic!("Details failed: {}", response.status_text());
    }
}