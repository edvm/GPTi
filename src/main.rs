mod config;
mod openai;
mod utils;
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

    /// Copy output to clipboard
    #[arg(long)]
    copy: bool,
}

#[derive(Deserialize)]
pub struct Prompt {
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
            let reply = openai.send_with_user_input(&prompt).await;
            if args.copy {
                utils::copy_to_clipboard(&reply);
                println!("Copied to clipboard!")
            }
        }
    }
}

fn read_prompts(config: &String) -> Result<Vec<Prompt>, std::io::Error> {
    let config_file = std::fs::read_to_string(config).unwrap();
    let config: toml::Value = toml::from_str(&config_file).unwrap();
    let prompts = config["prompt"].clone();
    let prompts: Vec<Prompt> = prompts.try_into().unwrap();
    Ok(prompts)
}
