use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct LoginResponse {
    pub jwtData: AuthToken,
}

// #[derive(Deserialize, Clone)]
// pub struct JwtData {
//     pub token: String,
//     #[serde(with = "chrono::serde::ts_seconds_option")]
//     pub expiry: Option<chrono::DateTime<chrono::Utc>>,
// }

#[derive(PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct AuthToken {
    pub token: String,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub expiry: Option<chrono::DateTime<chrono::Utc>>,
}

// #[derive(Clone)]
// pub struct AuthState {
//     pub token: Option<AuthToken>,
//     pub is_authenticated: bool,
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Plan {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionDetails {
    pub subscription_status: String,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub plan: Option<Plan>,
    pub cancel_at_period_end: bool,
}

// Extend AuthState to include subscription details
#[derive(Clone)]
pub struct AuthState {
    pub token: Option<AuthToken>,
    pub is_authenticated: bool,
    pub subscription: Option<SubscriptionDetails>,
}

impl AuthState {
    pub fn can_create_projects(&self) -> bool {
        if !self.is_authenticated {
            return false;
        }

        match &self.subscription {
            Some(sub) => matches!(sub.subscription_status.as_str(), "ACTIVE" | "TRIALING"),
            None => false,
        }
    }
}
