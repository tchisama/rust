use colored::*; // For colored terminal output
use crossterm::event::{self, KeyCode};
use reqwest::Client; // For making HTTP requests
use serde::{Deserialize, Serialize}; // For JSON serialization/deserialization
use std::env; // For accessing environment variables and command-line arguments
use std::process::{Command, Stdio}; // For spawning child processes // For handling input and terminal control

// Struct to represent the request payload for OpenAI API
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

// Struct to represent a single message
#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

// Struct to specify JSON response format
#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String,
}

// Struct to represent the OpenAI API response
#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

// Struct to represent a single choice (response)
#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

// Struct to represent the message content
#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

// Struct for the structured response
#[derive(Deserialize)]
struct StructuredResponse {
    final_command: String,
}

// Function to display the UI and handle user input

fn handle_user_input(command: &str) {
    loop {
        println!("\n[{}] Run | [{}] Exit", "R".green(), "E".red());

        if let Ok(true) = event::poll(std::time::Duration::from_secs(10)) {
            if let Ok(event::Event::Key(key_event)) = event::read() {
                match key_event.code {
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        println!("\nExecuting command: {}", command.green().bold());

                        // Spawn the process interactively
                        let mut child = Command::new("sh")
                            .arg("-c")
                            .arg(command)
                            .stdin(Stdio::inherit()) // Allow user interaction
                            .stdout(Stdio::inherit()) // Show real-time output
                            .stderr(Stdio::inherit()) // Show errors in real-time
                            .spawn()
                            .expect("Failed to execute command");

                        let _ = child.wait(); // Wait for the process to finish
                        return;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        println!("\nClosing...");
                        return;
                    }
                    _ => continue,
                }
            }
        }
    }
}
// The main function (async with tokio)
#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: shy <question>");
        return;
    }
    let question = args[1..].join(" ");

    println!("{}", "Thinking...".yellow());

    let request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a CLI assistant. Only return a JSON object with 'final_command' (string) for the CLI command . Do not include explanations or steps.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: question,
            },
        ],
        response_format: ResponseFormat {
            format_type: "json_object".to_string(),
        },
    };

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_body: OpenAIResponse =
            response.json().await.expect("Failed to parse response");

        let content = response_body.choices[0].message.content.trim();
        let structured_response: StructuredResponse =
            serde_json::from_str(content).unwrap_or_else(|_| {
                eprintln!("Failed to parse structured response.");
                StructuredResponse {
                    final_command: "echo 'Error: No command returned'".to_string(),
                }
            });

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
        let face = faces[rand::random::<usize>() % faces.len()];

        println!(
            "\n {} : {}",
            face.blue().bold(),
            structured_response.final_command.green().bold()
        );

        handle_user_input(&structured_response.final_command);
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
