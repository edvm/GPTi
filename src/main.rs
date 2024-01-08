use std::collections::HashMap;
use std::env;


use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole}, set_key,
};

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Operation {
    Mail,
}

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

/// A basic example
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The operation to perform
    #[arg(value_enum, short, long)]
    operation: Operation,
}

#[tokio::main]
async fn main() {
    let openai_key = read_openai_key().unwrap();
    set_key(openai_key);

    let args = Args::parse();

    match args.operation {
        Operation::Mail => op_mail().await,
    };
}

fn read_openai_key() -> Result<String, std::io::Error> {
    let env_name = OPENAI_API_KEY;

    let mut env_map = HashMap::new();
    for (key, val) in env::vars() {
        env_map.insert(key, val);
    }

    if !env_map.contains_key(env_name) {
        let error_message = format!("{} not found in .env file", env_name);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        ));
    } else {
        let key = env_map.get(env_name).unwrap();
        Ok(key.to_string())
    }
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

async fn op_mail() {
    println!("Write email content:");

    let input = get_user_input();
    let prompt = format!(
        r#"
    Craft a friendly email to share with your workmates based on the following text wrapped by triple backticks.
    The tone should be approachable, friendly but very concise, it should be fast to be read.
    Use language that is friendly yet professional, and encourage your team to share
    their thoughts on the matter discussed in the input text.
    ``` 
    {} 
    ```
    "#,
        input
    );

    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(prompt),
        name: None,
        function_call: None,
    }];

    let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
        .create()
        .await
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    println!("{}", returned_message.content.clone().unwrap().trim());

}
