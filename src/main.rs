mod config;
mod openai;
mod utils;
use clap::Parser;

use crate::config::Config;
use crate::openai::OpenAI;

/// GPTi
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Config file
    #[arg(short, long)]
    config: Option<String>,

    /// Prompt name to use (from config file)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Prompt text to use (inlined, text will be sent to OpenAI as is)
    #[arg(short, long)]
    text: Option<String>,
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

    let mut reply = String::new();

    // If text prompt is provided, use it and exit
    if let Some(text) = args.text {
        reply.push_str(&openai.send(&text).await);
    }

    // If prompt name is provided, use it
    if let Some(prompt_name) = args.prompt {
        for prompt in config.get_prompts() {
            if prompt.name == prompt_name {
                reply.push_str(&openai.send_with_user_input(&prompt).await);
            }
        }
    }

    // If reply is not empty, copy it to clipboard and exit
    if !reply.is_empty() {
        println!("\n{}", reply);
        utils::copy_to_clipboard(&reply);
        println!("\n-> output copied to clipboard\n");
        std::process::exit(0);
    }
}
