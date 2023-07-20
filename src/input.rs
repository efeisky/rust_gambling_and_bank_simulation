use std::io::{self, Write};

pub fn text_input(content: &str, is_safe: bool) -> String {
    print!("{}", content.to_string());
    io::stdout().flush().unwrap();
    match is_safe {
        false => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            input.trim().to_string()
        },
        true => {
            rpassword::read_password().unwrap()
        }
    }
}