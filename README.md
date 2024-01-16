# GPTi

GPTi is a tool built in Rust that lets you interact with the OpenAI GPT-3 model from the comfort of your command line.
Also, you can pre-save your own prompts using a gpti.toml config file, in order to avoid typing again and again the same prompts in the web brower (when using chatgpt)

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust: You can download Rust from [the official website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

```sh
git clone https://github.com/yourusername/gpti.git
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
```sh
➜  $ ./target/release/gpti --help
GPTi

Usage: gpti [OPTIONS] --prompt <PROMPT>

Options:
  -c, --config <CONFIG>  Config file
  -p, --prompt <PROMPT>  Prompt name to use (from config file)
      --copy             Copy output to clipboard
  -h, --help             Print help
  -V, --version          Print version
➜  GPTi git:(main) ✗ ./target/release/gpti 
```