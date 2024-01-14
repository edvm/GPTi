use std::io::Error;
use crate::yesno::ask_yes_no; // Update the import statement to reflect the changes in Rust 2018 module system

use directories::ProjectDirs;


pub struct Config{
    pub path: String,
}

impl Config {
    pub fn new(path: Option<String>) -> Self {

        // If path is provided, use it
        if let Some(path) = path {
            return Self { path };
        }

        // If path is not provided, use default 
        let default_path = Self::default_path().unwrap();
        Self { path: default_path }

    }

    pub fn default_path() -> Result<String, Error> {
        match ProjectDirs::from("tools", "cli",  "gpti") {
            Some(proj_dirs) => {
                let config_dir = proj_dirs.config_dir().to_str().unwrap();
                let path = format!("{}/gpti.toml", config_dir);
                Ok(path)
            },
            None => Err(Error::new(std::io::ErrorKind::Other, "Could not find config directory"))
        }
    }

    pub fn create_default(&self) -> Result<(), Error> {
        let msg = format!(r#"
        Config file not found at: {}
        It may be the first time you're running this program.
        I can help you with that by creating a default config file for you.
        Or you can give a path to an existing config file with the --config flag.
        Do you want me to create a default config file?
        "#, self.path);

        let create_file = ask_yes_no(&msg);

        if create_file{
            let path = std::path::Path::new(&self.path);
            let parent = path.parent().unwrap();
            std::fs::create_dir_all(parent)?;

            let config_path = Self::get_default_config();
            match std::fs::write(&self.path, config_path) {
                Ok(_) => {
                    println!("Config file created at: {}", self.path);
                    println!("Please add your OpenAI API key to the config file and try again.");
                    Ok(())
                }
                ,
                Err(e) => {
                    Err(e)
                }
            }
        } else {
            println!("No config file created, exiting...");
            Ok(())
        }
    }

    fn get_default_config() -> String {
        let default_config = r#"
[openai]
api_key = ""

[[prompts]]
name = "test"
text = "This is a test prompt"
description = "this is a test prompt"
"#;

        default_config.to_string()
    }

    pub fn doesnt_exists(&self) -> bool { 
        match std::fs::metadata(&self.path) {
            Ok(_) => false,
            Err(_) => true,
        }
    }
}