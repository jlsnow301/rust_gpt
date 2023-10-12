use crate::{chatbot::get_response, utils::read_input};

mod chatbot;
mod utils;

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
