// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This is our new command function that will be called from the frontend
#[tauri::command]
async fn test_immich_connection(server_url: String, api_key: String) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("API Key cannot be empty.".into());
    }

    // The CORRECT endpoint for validating a user's key, as you discovered.
    let validate_url = format!("{}/api/users/me", server_url.trim_end_matches('/'));

    let client = reqwest::Client::new();

    match client.get(&validate_url)
        .header("x-api-key", api_key)
        .header("Accept", "application/json")
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                // SUCCESS! The server gave a 200 OK response.
                Ok("Success! API Key is valid and authenticated.".into())
            } else if response.status().as_u16() == 401 {
                // The server gave a 401 Unauthorized, meaning the key is wrong.
                Err("API Key is not valid. Please check and try again.".into())
            }
            else {
                // Handle other errors.
                Err(format!("Server responded with status: {}", response.status()))
            }
        }
        Err(e) => {
            // A network error occurred
            Err(e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // This line tells Tauri about our new command so the frontend can call it
        .invoke_handler(tauri::generate_handler![test_immich_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}