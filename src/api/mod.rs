use serde::{Deserialize, Serialize};
use reqwest::Client;

const API_BASE_URL: &str = "https://backend-service-2zq0.onrender.com/api";

#[derive(Serialize)]
pub struct VerifyRequest {
    #[serde(rename = "ticketCode")]
    pub ticket_code: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ItemDetails {
    pub date: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub location: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TicketData {
    #[serde(rename = "bookingNumber")]
    pub booking_number: String,
    #[serde(rename = "ticketCode")]
    pub ticket_code: String,
    #[serde(rename = "itemTitle")]
    pub item_title: String,
    #[serde(rename = "itemType")]
    pub item_type: String,
    #[serde(default)]
    pub quantity: u32,
    #[serde(rename = "itemDetails")]
    pub item_details: ItemDetails,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VerifyResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<TicketData>,
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    #[serde(default)]
    errors: Vec<String>,
    #[serde(default)]
    message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Listing {
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub listing_type: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Deserialize, Debug)]
pub struct ListingsResponse {
    pub success: bool,
    pub data: Vec<Listing>,
}

pub async fn verify_ticket(ticket_code: String, token: String) -> Result<VerifyResponse, String> {
    let client = Client::new();
    // Using the Base URL provided by user
    let url = format!("{}/v1/vendor/tickets/validate", API_BASE_URL);
    
    // We need to match the backend expectation. 
    // If backend expects "ticketCode", we use that.
    let request = VerifyRequest { 
        ticket_code,
        notes: None, // Or we can pass this as an argument if needed
    };

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
        
        // Log to console
        web_sys::console::log_1(&format!("Backend Error ({}): {}", status, text).into());

        if status == 401 {
            return Err("You are not authorized to carry out this action".to_string());
        }

        // Try to parse specific error message
        if let Ok(error_res) = serde_json::from_str::<ApiErrorResponse>(&text) {
             if let Some(msg) = error_res.errors.first() {
                 return Err(msg.clone());
             }
             if !error_res.message.is_empty() {
                 return Err(error_res.message);
             }
        }

        return Err(format!("Server returned error: {}", status));
    }

    let verify_res = res.json::<VerifyResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
        
    Ok(verify_res)
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "mustResetPassword")]
    pub must_reset_password: bool,
    pub vendor: serde_json::Value,
}

pub async fn login(req: LoginRequest) -> Result<LoginResponse, String> {
    let client = Client::new();
    let url = format!("{}/v1/vendor/auth/login", API_BASE_URL);

    let res = client.post(url).json(&req).send().await.map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_else(|_| "Could not read response body".to_string());
        
        // Log to console
        web_sys::console::log_1(&format!("Login Error ({}): {}", status, text).into());

        // Try to parse specific error message
        if let Ok(error_res) = serde_json::from_str::<ApiErrorResponse>(&text) {
             if let Some(msg) = error_res.errors.first() {
                 return Err(msg.clone());
             }
             if !error_res.message.is_empty() {
                 return Err(error_res.message);
             }
        }
        
        // Fallback for 401 if no specific message
        if status == 401 {
             return Err("Invalid email or password".to_string());
        }

        return Err(format!("Login failed: {}", status));
    }

    let api_res = res.json::<ApiResponse<LoginResponse>>().await.map_err(|e| format!("Failed to parse response: {}", e))?;
    
    if api_res.success {
        Ok(api_res.data)
    } else {
        Err(api_res.message)
    }
}

#[derive(Serialize)]
pub struct ResetPasswordRequest {
    #[serde(rename = "currentPassword")]
    pub current_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
    #[serde(rename = "confirmNewPassword")]
    pub confirm_new_password: String,
}

pub async fn reset_password(req: ResetPasswordRequest, token: String) -> Result<(), String> {
    let client = Client::new();
    let url = format!("{}/v1/vendor/auth/reset-password", API_BASE_URL);

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&req)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Reset failed ({}): {}", status, error_text));
    }

    Ok(())
}

pub async fn fetch_listings(token: String) -> Result<Vec<Listing>, String> {
    let client = Client::new();
    let url = format!("{}/v1/vendor/listings", API_BASE_URL);

    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = res.status();
    
    if status.is_success() {
        let response = res.json::<ListingsResponse>().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response.data)
    } else {
        let error_text = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Failed to fetch listings: {}", error_text))
    }
}
