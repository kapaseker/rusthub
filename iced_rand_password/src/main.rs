#![windows_subsystem = "windows"]

use iced::alignment::Vertical;
use iced::widget::{Space, button, center, checkbox, row};
use iced::widget::{column, text};
use iced_rand_password::gen_pass::gen_pass_process::process_gen_pass;

#[derive(Debug, Clone)]
enum Message {
    Generate,
    Copy,
    ToggleUppercase(bool),
    ToggleLowercase(bool),
    ToggleNumber(bool),
    ToggleSymbol(bool),
    ChangeLen(u8),
}

const LEN_MIN: u8 = 8;
const LEN_MAX: u8 = 64;

struct Password {
    pass_word: String,
    score: u8,
    len: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
}

impl Default for Password {
    fn default() -> Self {
        Self {
            pass_word: String::new(),
            score: 0,
            len: LEN_MIN,
            uppercase: true,
            lowercase: true,
            number: true,
            symbol: true,
        }
    }
}

impl Password {
    fn update(&mut self, message: Message) {
        match message {
            Message::Generate => {
                let password = process_gen_pass(
                    self.len,
                    self.lowercase,
                    self.uppercase,
                    self.number,
                    self.symbol,
                )
                .unwrap_or("".to_string());

                let score = zxcvbn::zxcvbn(&password, &[]).score();

                self.pass_word = password;
                self.score = score.into();
            }
            Message::Copy => {
                let _ = iced::clipboard::write::<String>(self.pass_word.clone());
                println!("Copied! ");
            }
            Message::ToggleUppercase(value) => {
                self.uppercase = value;
            }
            Message::ToggleLowercase(value) => {
                self.lowercase = value;
            }
            Message::ToggleNumber(value) => {
                self.number = value;
            }
            Message::ToggleSymbol(value) => {
                self.symbol = value;
            }
            Message::ChangeLen(len) => {
                self.len = len;
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        // let button = button("Generate").on_press(Message::Generate);
        center(
            column![
                self.password_text(),
                self.score_text(),
                self.type_check_box(),
                self.len_slider(),
                self.operations_button(),
            ]
            .spacing(20),
        )
        .into()
    }

    fn operations_button(&self) -> iced::Element<'_, Message> {
        row![
            button("Generate").on_press(Message::Generate),
            // Space::with_width(100),
            // button("Copy").on_press(Message::Copy),
        ]
        .into()
    }

    fn password_text(&self) -> iced::Element<'_, Message> {
        column![text("Password").size(14), text(&self.pass_word),].into()
    }

    fn score_text(&self) -> iced::Element<'_, Message> {
        let score_text = if self.pass_word.is_empty() {
            text("".to_string())
        } else {
            text(self.score.to_string())
        };

        column![
            text("Security Score(higher is better)").size(14),
            score_text,
        ]
        .into()
    }

    fn type_check_box(&self) -> iced::Element<'_, Message> {
        row![
            checkbox("Uppercase", self.uppercase).on_toggle(Message::ToggleUppercase),
            checkbox("Lowercase", self.lowercase).on_toggle(Message::ToggleLowercase),
            checkbox("Number", self.number).on_toggle(Message::ToggleNumber),
            checkbox("Symbol", self.symbol).on_toggle(Message::ToggleSymbol),
        ]
        .spacing(16)
        .into()
    }

    fn len_slider(&self) -> iced::Element<'_, Message> {
        row![
            iced::widget::slider(LEN_MIN..=LEN_MAX, self.len, Message::ChangeLen).width(200),
            text(format!("Password Len: {}", self.len)),
        ]
        .spacing(12)
        .align_y(Vertical::Center)
        .into()
    }
}

fn main() -> iced::Result {
    println!("Hello, world!");
    iced::application("Generate Password", Password::update, Password::view).run()
}
