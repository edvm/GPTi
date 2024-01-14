pub fn ask_yes_no(prompt: &str) -> bool {
    println!("{}", prompt);
    println!("Type y or yes to confirm, anything else to cancel.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim() == "y" || input.trim() == "yes"
}