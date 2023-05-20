// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::warn;
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use tauri_plugin_log::LogTarget;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RandomOrgParams {
    api_key: String,
    n: u8,
    min: u8,
    max: u8,
    replacement: bool,
}

#[derive(Serialize, Deserialize)]
struct RandomOrgRequest {
    jsonrpc: String,
    method: String,
    params: RandomOrgParams,
    id: u8,
}

async fn request_random_bool() -> Result<serde_json::Value, Error> {
    let request_url = "https://api.random.org/json-rpc/4/invoke";
    let api_key =
        fs::read_to_string("../keys/random_org_api_key").expect("Error reading random.org api key");
    let body = RandomOrgRequest {
        jsonrpc: String::from("2.0"),
        method: String::from("generateIntegers"),
        params: RandomOrgParams {
            api_key,
            n: 1,
            min: 0,
            max: 1,
            replacement: true,
        },
        id: 1,
    };

    let body_str = serde_json::to_string(&body).expect("Error serializing body to string");
    let client = reqwest::Client::new();
    let json_response = client
        .post(request_url)
        .header("Content-Type", "application/json")
        .body(body_str)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(json_response)
}

async fn request_toss() -> Result<u64, String> {
    let json_toss = request_random_bool().await;
    match json_toss {
        Ok(json_value) => match json_value["result"]["random"]["data"][0].as_u64() {
            Some(value) => Ok(value),
            None => Err("Couldn't unwrap json value".to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}

#[tauri::command]
async fn process_toss() -> String {
    match request_toss().await {
        Ok(toss_value) => {
            let toss = if toss_value == 1 { "heads" } else { "tails" };
            format!("Hello, you tossed {}", toss)
        }
        Err(err) => format!("Errors happened: {}", err),
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![process_toss])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
