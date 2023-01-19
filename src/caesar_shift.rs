// 'A' in ASCII == 65

pub fn cipher(text: String, shift: String) -> String {
    let shift = (shift.trim().parse::<u128>().unwrap() % 26) as u8;

    text.to_uppercase().chars().map(|char| {
        if char != ' ' {
            (65 + (char as u8 + shift - 65) % 26) as char
        } else {
            char
        }
    }).collect()
}

pub fn decipher(text: String, shift: String) -> String {
    let shift = (shift.trim().parse::<u128>().unwrap() % 26) as u8;

    text.to_uppercase().chars().map(|char| {
        if char != ' ' {
            (65 + (char as u8 - shift + 65) % 26) as char
        } else {
            char
        }
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    // Cipher
    #[test]
    fn lower_case_to_upper() {
        assert_eq!(cipher(String::from("abc"), "1".to_string()), "BCD");
    }

    #[test]
    fn string_with_space_works() {
        assert_eq!(cipher(String::from("attack the castle"), "1".to_string()),
                   "BUUBDL UIF DBTUMF");
    }

    #[test]
    fn rotates_back_to_start() {
        assert_eq!(cipher(String::from("z"), "1".to_string()), "A");
    }

    #[test]
    fn key_goes_back_to_zero_cipher() {
        assert_eq!(cipher(String::from("A"), "26".to_string()), "A");
        assert_eq!(cipher(String::from("A"), "27".to_string()), "B");
        assert_eq!(cipher(String::from("Z"), "26".to_string()), "Z");
        assert_eq!(cipher(String::from("Z"), "27".to_string()), "A");
    }

    // Decipher
    #[test]
    fn rotates_to_end() {
        assert_eq!(decipher(String::from("A"), "1".to_string()), "Z");
    }

    #[test]
    fn key_goes_back_to_zero_decipher() {
        assert_eq!(decipher(String::from("A"), "26".to_string()), "A");
        assert_eq!(decipher(String::from("A"), "27".to_string()), "Z");
        assert_eq!(decipher(String::from("Z"), "26".to_string()), "Z");
        assert_eq!(decipher(String::from("Z"), "27".to_string()), "Y");
    }
}
