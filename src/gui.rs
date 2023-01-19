use crate::caesar_shift::{cipher, decipher};

use iced::{Alignment, Length, Sandbox};
use iced::widget::{button, column, container, row, text, text_input};


#[derive(Default)]
pub struct Scene {
    input_text: String,
    output_text: String,
    shift: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    ShiftChanged(String),
    EncryptPressed,
    DecryptPressed,
}


impl Scene {
    fn input_is_valid(&mut self) -> bool {
        let mut valid: bool = true;

        for char in self.input_text.trim().chars() {
            if !(char.is_ascii_alphabetic() || char.eq(&' ')) {
                self.input_text.clear();
                valid = false;
                break;
            }
        }
        valid
    }

    fn shift_is_valid(&mut self) -> bool {
        let mut valid: bool = true;

        for char in self.shift.trim().chars() {
            if !(char.is_numeric()) {
                self.shift.clear();
                valid = false;
                break;
            }
        }
        valid
    }
}


impl Sandbox for Scene {
    type Message = Message;

    fn new() -> Self {
        Scene::default()
    }

    fn title(&self) -> String {
        String::from("Caesar Shift")
    }

    
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::InputChanged(value) => {
                if self.input_is_valid() {
                    self.input_text = value;
                } else {
                    self.output_text = 
                        String::from("All characters should be in the latin alphabet")
                }
            },
            Message::EncryptPressed => {
                if self.input_is_valid()      &&
                  !self.input_text.is_empty() &&
                   self.shift_is_valid() {
                    self.output_text = cipher(self.input_text.clone(),
                                              self.shift.clone());
                    self.input_text = String::from("");
                } else if !self.input_is_valid() {
                    self.output_text = 
                        String::from("All characters should be in the latin alphabet")
                } else if self.input_text.is_empty() {
                    self.output_text = 
                        String::from("The text should not be empty.")
                }
            },
            Message::DecryptPressed => {
                if self.input_is_valid() {
                    self.output_text = decipher(self.input_text.clone(),
                                              self.shift.clone());
                    self.input_text = String::from("");
                } else if !self.input_is_valid() {
                    self.output_text = 
                        String::from("All characters should be in the latin alphabet")
                } else if !self.shift_is_valid() {
                    self.output_text = 
                        String::from("The shift value should be a number.")
                }
            },
            Message::ShiftChanged(shift) => {
                if self.shift_is_valid() {
                    self.shift = shift;
                } else {
                    self.output_text = 
                        String::from("The shift value should be a number.")
                }
            }
        }
    }


    fn view(&self) -> iced::Element<Message> {
        let user_input = text_input(
            "Text to encrypt...",
            &self.input_text,
            Message::InputChanged
        )
            .padding(10)
            .size(20);


        let shift_input = text_input(
            "Shift...",
            &self.shift,
            Message::ShiftChanged
        )
            .padding(10)
            .size(20)
            .width(Length::Units(80));
        

        let encrypt_button =
            button("Encrypt")
            .padding(10)
            .on_press(Message::EncryptPressed);


        let decrypt_button =
            button("Decrypt")
            .padding(10)
            .on_press(Message::DecryptPressed);
        
        
        let content = column![
            row![
                user_input,
                shift_input,
            ]
            .spacing(10)
            .width(Length::Units(800))
            .height(Length::Units(50))
            .align_items(Alignment::Center),
            row![
                encrypt_button,
                decrypt_button,
            ]
            .spacing(10)
            .padding(10),
            text(&self.output_text),
            ]
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center);


        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
