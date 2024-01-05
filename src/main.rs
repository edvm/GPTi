use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
enum Operation {
    Mail,
}

/// A basic example
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The operation to perform
    #[arg(value_enum, short, long)]
    operation: Operation,

}


fn main() {
    let args = Args::parse();
    let op = match args.operation {
        Operation::Mail => "mail",
    };
    println!("Operation: {}", op)
}
