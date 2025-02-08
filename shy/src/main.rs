use colored::*; // For colored terminal output
use reqwest::Client; // For making HTTP requests
use serde::{Deserialize, Serialize}; // For JSON serialization/deserialization
use std::env; // For accessing environment variables and command-line arguments

// Struct to represent the request payload for the OpenAI API
#[derive(Serialize)]
struct OpenAIRequest {
    model: String, // The model to use (e.g., "gpt-4")
    messages: Vec<Message>, // A list of messages in the conversation
    response_format: ResponseFormat, // Specify the response format as JSON
}

// Struct to represent a single message in the conversation
#[derive(Serialize)]
struct Message {
    role: String, // The role of the message sender (e.g., "system" or "user")
    content: String, // The content of the message
}

// Struct to specify the response format as JSON
#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    format_type: String, // The type of response format (e.g., "json_object")
}

// Struct to represent the response from the OpenAI API
#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>, // A list of choices (responses) from the API
}

// Struct to represent a single choice (response) from the API
#[derive(Deserialize)]
struct Choice {
    message: MessageResponse, // The message content of the response
}

// Struct to represent the message content in the API response
#[derive(Deserialize)]
struct MessageResponse {
    content: String, // The text content of the response
}

// Struct to represent the structured response from the API
#[derive(Deserialize)]
struct StructuredResponse {
    steps: Vec<Step>, // A list of steps in the solution
    final_answer: String, // The final answer
    final_command: Option<String>, // The final command to get the answer (optional)
    emoji: Option<String>, // A random emoji to display with the response (optional)
}

// Struct to represent a single step in the solution
#[derive(Deserialize)]
struct Step {
    explanation: String, // The explanation for the step
    output_command: String, // The output of the step
}

// The main function, marked as asynchronous with `tokio::main`
#[tokio::main]
async fn main() {
    // Step 1: Get the OpenAI API key from the environment variables
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    // Step 2: Get the question from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: shy <question>");
        return;
    }
    let question = args[1..].join(" ");

    // Step 3: Display a "thinking" message to indicate the app is processing the question
    println!("{}", "Thinking...".yellow());

    // Step 4: Create the request payload for the OpenAI API
    let request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(), // Use the GPT-4 model
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful math tutor. Guide the user through the solution step by step. Respond in JSON format with `steps` (an array of objects with `explanation` and `output_command` fields), a `final_answer` field, and optionally a `final_command` and `emoji` field if applicable.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: question,
            },
        ],
        response_format: ResponseFormat {
            format_type: "json_object".to_string(), // Request the response in JSON format
        },
    };

    // Step 5: Send the request to the OpenAI API using the `reqwest` crate
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    // Step 6: Parse the response from the OpenAI API
    if response.status().is_success() {
        let response_body: OpenAIResponse = response
            .json()
            .await
            .expect("Failed to parse response");

        // Extract the content of the first response choice
        let content = response_body.choices[0].message.content.trim();

        // Parse the JSON content into a structured response
        let structured_response: StructuredResponse = serde_json::from_str(content)
            .unwrap_or_else(|_| {
                eprintln!("Failed to parse structured response. Falling back to default.");
                StructuredResponse {
                    steps: vec![],
                    final_answer: "No answer provided.".to_string(),
                    final_command: None,
                    emoji: None,
                }
            });

        // Step 7: Display the response with a random face and formatted output
        let faces = ["(≧◡≦)", "(*^▽^*)", "(◕3◕)", "(^◡^)", "(∪ ◡ ∪)"]; // List of random faces
        let face = faces[rand::random::<usize>() % faces.len()]; // Pick a random face

        // Print the steps with explanations and outputs
        println!("\n {} : Steps to solve the problem:", face.blue().bold());
        for (i, step) in structured_response.steps.iter().enumerate() {
            println!(
                "  {}. {}: {}",
                i + 1,
                step.explanation.green(),
                step.output_command.blue()
            );
        }

        let emoji = structured_response.emoji.unwrap_or("".to_string());
        let final_command = structured_response.final_command.unwrap_or("".to_string());

        // Print the final answer
        println!("\n {} {}: {} {}", 
            face.blue().bold(),
            emoji,
            structured_response.final_answer.green().bold(),
            final_command.blue().bold()
        );

    } else {
        // If the request failed, display the error message from the API
        eprintln!(
            "Error: {}",
            response.text().await.expect("Failed to read error response")
        );
    }
}
