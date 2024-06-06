use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO;

use openai_api_rs::v1::message::{CreateMessageRequest, MessageRole};
use openai_api_rs::v1::run::CreateRunRequest;
use openai_api_rs::v1::thread::CreateThreadRequest;

pub fn chat(message: String, api_token: String) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let client = Client::new(api_token);

    println!("{}", &message);

    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(message),
            name: None,
        }],
    );

    let result = client.chat_completion(req)?;
    let string = &result.choices[0].message.content;
    // println!("Content: {:?}", result.choices[0].message.content);
    // println!("Response Headers: {:?}", result.headers);

    Ok(string.clone())
}

// #[derive(Clone)]
pub struct ChatSession {
    _client: Client,
    pub thread_id: String,
    // pub message: String,
}

impl ChatSession {
    pub fn create_conversation(api_token: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new(api_token);

        let thread_req = CreateThreadRequest::new();
        let thread_result = client.create_thread(thread_req)?;
        // println!("{:?}", thread_result.id.clone());

        Ok(ChatSession {
            _client: client,
            thread_id : thread_result.id.clone(),
        })
    }

}

pub fn talk(assistan_id: String, thread_id: String, message: String, api_token: String) -> Result<String, Box<dyn std::error::Error>>{
    let client = Client::new(api_token);

    let message_req = CreateMessageRequest::new(
        MessageRole::user,
        message, // se le manda el mensaje
    );

    let message_result = client.create_message(thread_id.clone(), message_req)?;
    println!("{:?}", message_result.id.clone());

    let run_req = CreateRunRequest::new(assistan_id);
    let run_result = client.create_run(thread_id.clone(), run_req)?;

    loop {
        let run_result = client
        .retrieve_run(thread_id.clone(), run_result.id.clone())
        .unwrap();
    if run_result.status == "completed" {
        break;
    } else {
            println!("waiting...");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    let first_message = if let Some(data) = client.list_messages(thread_id.clone())?.data.first() {
        if let Some(content) = data.content.first() {
            content.text.value.clone()
        } else {
            String::new() // If there's no content in the first message, return an empty string
        }
    } else {
        String::new() // If there are no messages, return an empty string
    };

    Ok(first_message)
}
