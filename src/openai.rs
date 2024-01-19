use crate::{config::Prompt, utils};
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};
use spinners::{Spinner, Spinners};
use std::collections::HashMap;
use toml;

pub struct OpenAI {}

impl OpenAI {
    pub fn new(config_path: &String) -> OpenAI {
        let api_key = Self::read_api_key(&config_path);
        set_key(api_key.clone());
        OpenAI {}
    }

    pub fn read_api_key(config_path: &String) -> String {
        let config_file = std::fs::read_to_string(config_path).unwrap();
        let config: toml::Value = toml::from_str(&config_file).unwrap();
        let openai = config["openai"].clone();
        let config: HashMap<String, String> = openai.try_into().unwrap();
        config["api_key"].clone()
    }

    pub async fn send(&self, prompt: &String) -> String {
        let mut p = String::new();
        p.push_str(&prompt);
        let reply = get_openai_reply(&p).await;
        reply.content.unwrap()
    }

    pub async fn send_with_user_input(&self, prompt: &Prompt) -> String {
        let mut p = String::new();
        p.push_str(&prompt.text);

        println!("{}", prompt.description);
        let user_input = utils::get_user_input();
        let data = format!("```{}```", user_input);
        p.push_str(&data);

        let reply = self.send(&p).await;
        reply
    }
}

pub async fn get_openai_reply(prompt: &String) -> ChatCompletionMessage {
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(prompt.clone()),
        name: None,
        function_call: None,
    }];

    let mut spinner = Spinner::new(Spinners::Dots9, "Sending prompt to OpenAI...".into());

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap();

    spinner.stop();

    return chat_completion.choices.first().unwrap().message.clone();
}
