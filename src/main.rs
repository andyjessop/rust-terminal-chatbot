use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::stdin;

#[derive(Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv::dotenv().ok();
    let client = Client::new(); // Initialize the HTTP client outside the loop.
    let mut conversation = Conversation {
        messages: Vec::new(),
    };

    println!("\nWhat do you want to talk about today?\n");

    loop {
        // Prompt user for their question
        println!("\nYou:");
        let mut question = String::new();
        stdin()
            .read_line(&mut question)
            .map_err(|e| e.to_string())?;
        let question = question.trim(); // Remove any newline characters

        if question.eq_ignore_ascii_case("exit") {
            break; // Exit if the user types 'exit'
        }

        // Add user's question to the conversation
        conversation.messages.push(Message {
            role: "user".to_string(),
            content: question.to_string(),
        });

        // Get response from OpenAI and add to the conversation
        let response = ask_openai(&client, &mut conversation).await?;
        println!("\nAI:\n{}", response);
    }

    Ok(())
}

async fn ask_openai(client: &Client, conversation: &mut Conversation) -> Result<String, String> {
    let request_body = OpenAIChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: conversation.messages.clone(),
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
            ),
        )
        .json(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_body = response
        .json::<OpenAIChatResponse>()
        .await
        .map_err(|e| e.to_string())?;
    if let Some(choice) = response_body.choices.last() {
        conversation.messages.push(Message {
            role: "assistant".to_string(),
            content: choice.message.content.clone(),
        });
        Ok(choice.message.content.clone())
    } else {
        Err("No response from AI".to_string())
    }
}
