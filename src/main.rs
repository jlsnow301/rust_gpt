use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    loop {
        let user_input = read_input("You: ");

        if user_input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        let response = get_response(&user_input);
        println!("Bot: {}", response);
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn get_response(input: &str) -> &str {
    match input.trim().to_lowercase().as_str() {
        "hello" => "Hello there!",
        "how are you?" => "I'm doing well, how about you?",
        "what is your name?" => "My name is Rusty!",
        "exit" => "Bye!",
        _ => "Sorry, I didn't understand that.",
    }
}
