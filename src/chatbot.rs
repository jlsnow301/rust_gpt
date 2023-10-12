pub fn get_response(input: &str) -> &str {
    match input.trim().to_lowercase().as_str() {
        "hello" => "Hello there!",
        "how are you?" => "I'm doing well, how about you?",
        "what is your name?" => "My name is Rusty!",
        "exit" => "Bye!",
        _ => "Sorry, I didn't understand that.",
    }
}
