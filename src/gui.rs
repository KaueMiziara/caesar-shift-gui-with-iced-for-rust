use crate::caesar_shift::cipher;

use iced::{Alignment, Length, Sandbox};
use iced::widget::{button, column, container, row, text, text_input};


#[derive(Default)]
pub struct Scene {
    input_text: String,
    output_text: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    EncryptPressed,
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
}


impl Sandbox for Scene {
    type Message = Message;

    fn new() -> Self {
        Scene::default()
    }

    fn title(&self) -> String {
        String::from("Caesar Shift")
    }


    // TODO decrypt functionality
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
                if self.input_is_valid() {
                    self.output_text = cipher(self.input_text.clone(), 2);
                    self.input_text = String::from("");
                } else {
                    self.output_text = 
                        String::from("All characters should be in the latin alphabet")
                }
            },
        }
    }


    // TODO decrypt button
    fn view(&self) -> iced::Element<Message> {
        let text_input = text_input(
            "Text to encrypt...",
            &self.input_text,
            Message::InputChanged
        )
            .padding(10)
            .size(20);

        
        let button =
            button("Encrypt")
            .padding(10)
            .on_press(Message::EncryptPressed);


        let content = column![
            row![
                text_input,
                button,
            ]
            .spacing(10)
            .width(Length::Units(800))
            .height(Length::Units(50))
            .align_items(Alignment::Center),
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
