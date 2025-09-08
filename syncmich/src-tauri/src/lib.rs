// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This is our new command function that will be called from the frontend
#[tauri::command]
async fn test_immich_connection(server_url: String, api_key: String) -> Result<String, String> {
    // Construct the URL for the ping endpoint
    // This is the CORRECT line
    let ping_url = format!("{}/api/server/version", server_url.trim_end_matches('/'));

    // Use the 'reqwest' library to make the HTTP request
    match reqwest::get(&ping_url).await {
        Ok(response) => {
            if response.status().is_success() {
                // Success
                Ok("Connection successful! Server is reachable.".into())
            } else {
                // The server responded with an error code (like 404)
                Err(format!("Server responded with status: {}", response.status()))
            }
        }
        Err(e) => {
            // A network error occurred (like the server being offline)
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