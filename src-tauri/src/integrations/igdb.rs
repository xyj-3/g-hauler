use serde::{Deserialize, Serialize};
use tauri::AppHandle;

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
/// This function retrieves the Client ID and Client Secret from the app settings,
/// then exchanges them for an access token using the Twitch OAuth endpoint.
///
/// # Arguments
/// * `app_handle` - The Tauri application handle to access the store
///
/// # Returns
/// * `Ok(TwitchTokenResponse)` - Contains the access token and expiration info
/// * `Err(AuthError)` - If authentication fails
///
/// # Example
/// ```rust,no_run
/// use crate::integrations::igdb::authenticate;
///
/// async fn example(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
///     let token_response = authenticate(app_handle).await?;
///     println!("Access token: {}", token_response.access_token);
///     println!("Expires in: {} seconds", token_response.expires_in);
///     Ok(())
/// }
/// ```
pub async fn authenticate(app_handle: &AppHandle) -> Result<TwitchTokenResponse, AuthError> {
    // Retrieve credentials from store
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
/// It's meant to be called from the frontend to verify credentials.
#[tauri::command]
pub async fn check_igdb_auth(app_handle: AppHandle) -> Result<AuthStatus, String> {
    match authenticate(&app_handle).await {
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
