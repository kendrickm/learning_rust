use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(env::var("OPENAI_API_TOKEN").unwrap().to_string());
    let req = ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: String::from("I want my output to be in JSON in the format of {[day:meal]} My family is 2 adults and 1 toddler. We don't eat beef or pork. Give me a week's dinner menu."),
        }],
    };
    let result = client.chat_completion(req).await?;
    print!("{:?}", result.choices[0].message.content.trim());

    Ok(())
}
