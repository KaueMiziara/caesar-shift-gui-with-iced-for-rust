use std::io;

pub fn cipher(text: String, shift: u8) -> String {
    // 'A' in ASCII == 65
    text.to_uppercase().chars().map(|char| {
        if char != ' ' {
            (65 + (char as u8 + shift - 65) % 26) as char
        } else {
            char
        }
    }).collect()
}

pub fn decipher(_key: u8) -> String {
    let text = read_input();

    text
}


pub fn read_input() -> String {
    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let mut valid: bool = true;

                for char in buffer.trim().chars() {
                    if !(char.is_ascii_alphabetic() || char.eq(&' ')) {
                        println!("All characters should be in the latin alphabet, \
                        got {char}");
                        
                        buffer.clear();
                        valid = false;
                        
                        break;
                    }
                }
                if valid == true {
                    return buffer.trim().to_string();
                }
            }
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_case_to_upper_abc() {
        assert_eq!(cipher(String::from("abc"), 1), "BCD");
    }

    #[test]
    fn string_with_space_works() {
        assert_eq!(cipher(String::from("attack the castle"), 1), "BUUBDL UIF DBTUMF");
    }
}
