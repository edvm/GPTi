# GPTi

GPTi is a tool built in Rust that lets you interact with the OpenAI GPT-3 model from the comfort of your command line. Additionally, you can pre-save your own prompts using a *gpti.toml* config file in order to avoid typing the same prompts over and over again in the web browser when using chatgpt.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from [the official website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

```sh
git clone https://github.com/edvm/GPTi.git
cd gpti
```

2. Build the project:
```sh
cargo build --release
```

3. Run it (It'll ask you to create a default config file if first time):
```sh
./target/release/gpti -p init
```

### Usage

GPTI is configured by a *gpti.toml* file which is like:
```sh
[openai]
api_key = "your-api-key"

[[prompt]]
name = "grammar"
text = '''
Fix any grammar issues in the following text wrapped by triple backticks.
'''
description = "Fix grammar issues in the following text. Type EOF to finish:"

[[prompt]]
name = "prompt-1"
text = '''This is prompt-1. Type EOF to finish:"'''
description = "Some description"

[[prompt]]
name = "prompt-2"
text = '''This is prompt-2. Type EOF to finish:"'''
description = "Some other description"
```

This file can have any numbers of *prompts*. Later, you'll pass the *name* value along with the *-p* command line argument. For example:
```sh
➜  GPTi git:(main) ✗ ./target/release/gpti -p grammar 
Fix grammar issues in the following text. Type EOF to finish:
This a pencil
EOF
⢺ Sending prompt to OpenAI...
This is a pencil.
```

Check the help for options:
```sh
➜  $ ./target/release/gpti --help
GPTi

Usage: gpti [OPTIONS] --prompt <PROMPT>

Options:
  -c, --config <CONFIG>  Config file
  -p, --prompt <PROMPT>  Prompt name to use (from config file)
  -h, --help             Print help
  -V, --version          Print version
```
