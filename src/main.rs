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
    prompt: String,
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

    for prompt in config.get_prompts() {
        if prompt.name == args.prompt {
            let reply = openai.send_with_user_input(&prompt).await;
            utils::copy_to_clipboard(&reply);
            println!("-> output copied to clipboard")
        }
    }
}
