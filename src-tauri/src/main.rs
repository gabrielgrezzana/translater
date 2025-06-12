// Prevent additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use serde_json::Value;

#[tauri::command]
async fn translate_simple(text: &str) -> Result<String, String> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=auto&tl=pt&dt=t&q={}", 
        urlencoding::encode(text)
    );
    
    let response = reqwest::get(&url).await
        .map_err(|e| e.to_string())?;
    
    let json: Value = response.json().await
        .map_err(|e| e.to_string())?;
    
    if let Some(translation) = json[0][0][0].as_str() {
        Ok(translation.to_string())
    } else {
        Err("Translation failed".to_string())
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}! You've been greeted from Rust!", name)
}

// Nova função para tradução
#[tauri::command]
fn translate_text(text: &str, mode: &str) -> String {
    match mode {
        "simple" => {
            // Lógica para tradução simples
            format!("Simple translation of: '{}'", text)
        },
        "compose" => {
            // Lógica para tradução composta
            format!("Compose translation of: '{}'", text)
        },
        _ => {
            "Invalid translation mode".to_string()
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, translate_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}