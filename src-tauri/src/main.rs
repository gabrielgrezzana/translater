// Prevent additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;



#[derive(Serialize)]
struct GroqRequest {
    messages: Vec<Message>,
    model: String,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[tauri::command]
async fn translate_simple(text: &str) -> Result<String, String> {
    let url = format!(
        "https://translate.googleapis.com/translate_a/single?client=gtx&sl=pt&tl=en&dt=t&q={}", 
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

#[tauri::command]
async fn translate_with_custom_prompt(text: &str, custom_script: &str) -> Result<String, String> {
    // Carrega as variáveis do .env
    dotenv().ok();
    
    // Pega a variável API_KEY
    let api_key = env::var("API_KEY")
        .map_err(|_| "API_KEY não encontrada no .env".to_string())?;


    println!("=== INICIANDO TRADUÇÃO ===");
    println!("Text: {}", text);
    println!("Custom script: {}", custom_script);
    
    let client = reqwest::Client::new();
    
    let full_prompt = format!(
        "{}\n\nTexto para traduzir: {}", 
        custom_script, 
        text
    );
    
    println!("Full prompt: {}", full_prompt);
    
    let request = GroqRequest {
        messages: vec![Message {
            role: "user".to_string(),
            content: full_prompt,
        }],
        model: "meta-llama/llama-4-scout-17b-16e-instruct".to_string(),
        max_tokens: 500,
    };

    // Log do request serializado
    match serde_json::to_string_pretty(&request) {
        Ok(json_str) => println!("Request JSON: {}", json_str),
        Err(e) => println!("Erro ao serializar request: {}", e),
    }

    println!("=== FAZENDO REQUEST ===");
    
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key)) 
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            println!("Erro no request: {}", e);
            e.to_string()
        })?;

    println!("Status da response: {}", response.status());
    
    let response_text = response.text().await.map_err(|e| {
        println!("Erro ao ler response text: {}", e);
        e.to_string()
    })?;
    
    println!("Response completa: {}", response_text);
    
    Ok(response_text)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, translate_text, translate_simple, translate_with_custom_prompt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}