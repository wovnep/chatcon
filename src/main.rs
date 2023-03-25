#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde_json::{ Value, json };
use client::youtube::channel::{find_channel};

mod client;
mod utils;

#[tauri::command]
async fn search_channel(query: String) -> Value {
    let search_response = find_channel(query).await;
    let main_res  = search_response.unwrap_or_else(|_err|{ panic!("Reqwest failed")}).json().await;
    let value = match main_res {
        Ok(success_value) => {
            success_value
        },
        Err(error) => {
            let status_code = error.status().unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR);
            let error_message = json!({
                "error": true,
                "status": status_code.as_u16()
            });
            error_message
        }
    };
    value
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_channel])
        .run(tauri::generate_context!())
        .expect("unable to run chatcon");
}