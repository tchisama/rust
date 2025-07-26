use colored::*; // For colored terminal output
use reqwest::Client; // For making HTTP requests
use serde::{Deserialize, Serialize}; // For JSON serialization/deserialization
use std::env; // For accessing environment variables and command-line arguments
use rand::Rng;

// Struct to represent the request payload for Gemini API
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// Struct to represent the Gemini API response
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

// The main function (async with tokio)
#[tokio::main]
async fn main() {
    let api_key = "AIzaSyA05-GGz5IEpiKu1PwoiUeMzFi4BlEETiU";

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: shy <question>");
        return;
    }
    let question = args[1..].join(" ");

    println!("{}", "Thinking...".yellow());

    let request = GeminiRequest {
        contents: vec![
            Content {
                parts: vec![
                    Part {
                        text: "You are a friendly assistant. Respond with a short, friendly message.".to_string(),
                    },
                    Part {
                        text: question,
                    },
                ],
            },
        ],
    };

    let client = Client::new();
    let response = client
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_body: GeminiResponse =
            response.json().await.expect("Failed to parse response");

        let content = &response_body.candidates[0].content.parts[0].text;

        let faces = [
            "(≧◡≦)",
            "(*^▽^*)",
            "(◕3◕)",
            "(^◡^)",
            "(∪ ◡ ∪)",
            "(✿◠‿◠)",
            "(◕‿◕✿)",
            "(◠‿◠✿)",
            "(◕‿-) ✧",
            "(◕‿◕)",
        ];
        let mut rng = rand::thread_rng();
        let face = faces[rng.gen_range(0..faces.len())];

        println!(
            "\n {} : {}",
            face.blue().bold(),
            content.green().bold()
        );
    } else {
        eprintln!(
            "Error: {}",
            response
                .text()
                .await
                .expect("Failed to read error response")
        );
    }
}
