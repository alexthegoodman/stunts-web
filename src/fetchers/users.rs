use std::time::Duration;

use chrono::{DateTime, Local};
use leptos::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn login_user(
    email: String,
    password: String,
) -> LoginResponse {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build().expect("Couldn't build request");

    let response = client
        .post("http://localhost:3000/api/auth/login")
        .json(&LoginRequest { email, password })
        .send()
        .await.expect("Couldn't login user");

    // if response.status().is_success() {
        let login_response = response.json::<LoginResponse>().await.expect("Couldn't login user");
        login_response
    // } 
    // else {
    //     let error_text = response.text().await?;
    //     Err(error_text.into())
    // }
}

// Function to fetch subscription details
pub async fn fetch_subscription_details(
    token: &str,
) -> SubscriptionDetails {
    let client = Client::new();

    let response = client
        .get("http://localhost:3000/api/subscription/details")
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await.expect("Couldn't fetch subscription details");

    // if response.status().is_success() {
        let details = response.json::<SubscriptionDetails>().await.expect("Couldn't fetch subscription details");
        details
    // } 
    // else {
    //     Err(response.text().await?.into())
    // }
}