use std::io;

pub fn cipher(_key: u8) -> String {
    let text = read_input();

    text.trim().to_string()
}

pub fn decipher(_key: u8) -> String {
    let text = read_input();

    text.trim().to_string()
}


fn read_input() -> String {
    let mut buffer = String::new();

    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                let mut alpha: bool = true;

                for char in buffer.trim().chars() {
                    if !(char.is_ascii_alphabetic() || char.eq(&' ')) {
                        println!("All characters should be in the latin alphabet, \
                        got {char}");
                        
                        buffer.clear();
                        alpha = false;
                        
                        break;
                    }
                }
                if alpha == true {
                    return buffer;
                }
            }
            Err(_) => {
                println!("Invalid input. Try again.");
                continue;
            }
        }
    };
}
