use std::{io, ops::Add};

pub fn cipher(_key: u8) -> String {
    let text = read_input();

    text.trim().to_string().add("rhs")
}

pub fn decipher(_key: u8) -> String {
    let text = read_input();

    text
}


fn read_input() -> String {
    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                for char in buffer.trim().chars() {
                    if !char.is_ascii_alphabetic() {
                        println!("All characters must be in the latin alphabet.");
                        buffer.clear();
                        break;
                    } else {
                        return buffer;
                    }
                }
            }
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        }
    };
}
