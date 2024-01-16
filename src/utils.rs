use cli_clipboard;

pub fn ask_yes_no(prompt: &str) -> bool {
    println!("{}", prompt);
    println!("Type y or yes to confirm, anything else to cancel.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim() == "y" || input.trim() == "yes"
}


pub fn get_user_input() -> String {
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

pub fn copy_to_clipboard(text: &str) {
    cli_clipboard::set_contents(text.to_string()).unwrap();
}