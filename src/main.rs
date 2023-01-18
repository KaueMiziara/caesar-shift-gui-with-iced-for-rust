use caesar_shift_gui::caesar_shift::{cipher, decipher, read_input};
use caesar_shift_gui::gui;
use iced::Sandbox;

fn main() {
    let text = read_input();
    let encrypted = cipher(text, 2);
    println!("{encrypted}");

    let encrypted = decipher(encrypted, 2);
    println!("{encrypted}");
}
