use std::collections::HashMap;
use toml;
use spinners::{Spinner, Spinners};

use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key,
};

use clap::Parser;
use serde::Deserialize;

/// GPTi
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Config file
    #[arg(short, long)]
    config: Option<String>,

    /// Prompt name to use (from config file)
    #[arg(short, long)]
    prompt: String,
}

#[derive(Deserialize)]
struct Prompt {
    text: String,
    name: String,
    description: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Some(config) = args.config {
        let api_key = read_openai_key(&config).unwrap();
        set_key(api_key);

        for prompt in read_prompts(&config).unwrap() {
            if prompt.name == args.prompt {
                exec_prompt(&prompt).await;
            }
        }
    }
}

async fn exec_prompt(prompt: &Prompt) {
    let mut p = String::new();
    p.push_str(&prompt.text);

    println!("{}", prompt.description);
    let user_input = get_user_input();
    let data = format!("```{}```", user_input);
    p.push_str(&data);

    let reply = get_openai_reply(&p).await;

    println!("\n{}", reply.content.unwrap());
}

fn read_openai_key(config: &String) -> Result<String, std::io::Error> {
    let config_file = std::fs::read_to_string(config).unwrap();
    let config: toml::Value = toml::from_str(&config_file).unwrap();
    let openai = config["openai"].clone();
    let config: HashMap<String, String> = openai.try_into().unwrap();
    Ok(config["api_key"].clone())
}

fn read_prompts(config: &String) -> Result<Vec<Prompt>, std::io::Error> {
    let config_file = std::fs::read_to_string(config).unwrap();
    let config: toml::Value = toml::from_str(&config_file).unwrap();
    let prompts = config["prompt"].clone();
    let prompts: Vec<Prompt> = prompts.try_into().unwrap();
    Ok(prompts)
}

fn get_user_input() -> String {
    let mut input = String::new();

    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.trim() == "EOF" {
            break;
        }
        input.push_str(&line);
    }
    input
}

async fn get_openai_reply(prompt: &String) -> ChatCompletionMessage {

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
