mod config;
mod openai;
mod yesno;
use toml;

use crate::config::Config;
use crate::openai::OpenAI;

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
    let config = Config::new(args.config);

    if config.doesnt_exists() {
        match config.create_default() {
            Ok(_) => std::process::exit(0),
            Err(e) => {
                println!("Could not create config file: {}", e);
                std::process::exit(1);
            }
        }
    }

    let openai = OpenAI::new(&config.path);

    for prompt in read_prompts(&config.path).unwrap() {
        if prompt.name == args.prompt {
            exec_prompt(&prompt, &openai).await;
        }
    }
}

async fn exec_prompt(prompt: &Prompt, openai: &OpenAI) {
    let mut p = String::new();
    p.push_str(&prompt.text);

    println!("{}", prompt.description);
    let user_input = get_user_input();
    let data = format!("```{}```", user_input);
    p.push_str(&data);

    let reply = openai.send(&p).await;
    println!("\n{}", reply);
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
