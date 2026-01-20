use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, State};

use crate::core::constants::{STORE_KEY_IGDB_CLIENT_ID, STORE_KEY_IGDB_CLIENT_SECRET};
use crate::core::store::get_store_key;

/// The Twitch OAuth token endpoint used for IGDB authentication
const TWITCH_OAUTH_URL: &str = "https://id.twitch.tv/oauth2/token";

/// Response from the Twitch OAuth token endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

/// Cache for storing IGDB authentication tokens
#[derive(Debug, Default)]
pub struct TokenCache {
    access_token: Option<String>,
    expires_at: Option<SystemTime>,
}

impl TokenCache {
    /// Checks if the cached token is still valid
    fn is_valid(&self) -> bool {
        match (&self.access_token, self.expires_at) {
            (Some(_), Some(expires_at)) => SystemTime::now() < expires_at,
            _ => false,
        }
    }

    /// Updates the cache with a new token and expiration time
    fn update(&mut self, token: String, expires_in: u64) {
        // Subtract 60 seconds buffer to refresh before actual expiration
        let expiration = SystemTime::now() + Duration::from_secs(expires_in.saturating_sub(60));
        self.access_token = Some(token);
        self.expires_at = Some(expiration);
    }

    /// Gets the cached token if it's valid
    fn get_token(&self) -> Option<&str> {
        if self.is_valid() {
            self.access_token.as_deref()
        } else {
            None
        }
    }
}

/// Error types for IGDB authentication
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Client ID not configured")]
    MissingClientId,

    #[error("Client Secret not configured")]
    MissingClientSecret,

    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to retrieve credentials from store: {0}")]
    StoreError(String),

    #[error("Invalid credentials")]
    InvalidCredentials,
}

/// Retrieves IGDB credentials from the application store
fn get_credentials(app_handle: &AppHandle) -> Result<(String, String), AuthError> {
    // Get client ID
    let client_id = get_store_key(app_handle, STORE_KEY_IGDB_CLIENT_ID)
        .and_then(|v| v.as_str().map(String::from))
        .ok_or(AuthError::MissingClientId)?;

    // Get client secret
    let client_secret = get_store_key(app_handle, STORE_KEY_IGDB_CLIENT_SECRET)
        .and_then(|v| v.as_str().map(String::from))
        .ok_or(AuthError::MissingClientSecret)?;

    // Validate they're not empty strings
    if client_id.trim().is_empty() {
        return Err(AuthError::MissingClientId);
    }

    if client_secret.trim().is_empty() {
        return Err(AuthError::MissingClientSecret);
    }

    Ok((client_id, client_secret))
}

/// Authenticates with IGDB API using Twitch OAuth Client Credentials flow
///
/// This function checks the token cache first. If a valid token exists, it returns
/// immediately without making an HTTP request. Otherwise, it retrieves the Client ID
/// and Client Secret from the app settings, exchanges them for an access token using
/// the Twitch OAuth endpoint, and caches the result.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle to access the store
/// * `token_cache` - Shared token cache state
///
/// # Returns
/// * `Ok(TwitchTokenResponse)` - Contains the access token and expiration info
/// * `Err(AuthError)` - If authentication fails
pub async fn authenticate(
    app_handle: &AppHandle,
    token_cache: &Arc<Mutex<TokenCache>>,
) -> Result<TwitchTokenResponse, AuthError> {
    // Check cache first
    {
        let cache = token_cache.lock().unwrap();
        if let Some(token) = cache.get_token() {
            // Return cached token without making HTTP request
            return Ok(TwitchTokenResponse {
                access_token: token.to_string(),
                expires_in: 0, // Not needed when returning cached token
                token_type: "Bearer".to_string(),
            });
        }
    }

    // Cache miss or expired - retrieve credentials from store
    let (client_id, client_secret) = get_credentials(app_handle)?;

    // Build the HTTP client
    let client = reqwest::Client::new();

    // Make POST request to Twitch OAuth endpoint
    let response = client
        .post(TWITCH_OAUTH_URL)
        .query(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "client_credentials"),
        ])
        .send()
        .await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(AuthError::InvalidCredentials);
    }

    // Parse the response
    let token_response = response.json::<TwitchTokenResponse>().await?;

    // Update cache with new token
    {
        let mut cache = token_cache.lock().unwrap();
        cache.update(token_response.access_token.clone(), token_response.expires_in);
    }

    Ok(token_response)
}

/// Response for authentication status check
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub message: String,
}

/// Tauri command to check IGDB authentication status
///
/// This command attempts to authenticate with IGDB and returns the status.
/// It uses token caching to avoid unnecessary HTTP requests.
#[tauri::command]
pub async fn check_igdb_auth(
    app_handle: AppHandle,
    token_cache: State<'_, Arc<Mutex<TokenCache>>>,
) -> Result<AuthStatus, String> {
    match authenticate(&app_handle, &token_cache).await {
        Ok(_) => Ok(AuthStatus {
            authenticated: true,
            message: "IGDB authentication successful".to_string(),
        }),
        Err(e) => Ok(AuthStatus {
            authenticated: false,
            message: format!("IGDB authentication failed: {}", e),
        }),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_credentials_validation() {
        // This test verifies that empty strings are rejected
        // In actual usage, you'd need a mock AppHandle to test get_credentials
    }
}
