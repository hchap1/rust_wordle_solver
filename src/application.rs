use std::array;

use crate::wordle::{Colour, WordleGame};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    UpdateInput(char),
    BackspaceInput,
    AttemptSubmit,
    Toggle(usize, usize),
    Clear
}

#[derive(Default)]
pub struct Application {
    submitted: Vec<([char; 5], [Colour; 5])>,
    input: String
}

impl Application {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::UpdateInput(new_value) => {
                self.input.push(new_value);
                iced::Task::none()
            }

            Message::BackspaceInput => {
                self.input.pop();
                iced::Task::none()
            }

            Message::AttemptSubmit => {
                if self.input.len() != 5 {
                    return iced::Task::none();
                }
                let chars: Vec<char> = self.input.drain(..).collect();
                let chars: [char; 5] = array::from_fn(|n| chars[n]);
                self.submitted.push((
                    chars,
                    [Colour::Gray; 5]
                ));
                iced::Task::none()
            }

            Message::Toggle(row, col) => {
                self.submitted[col].1[row] = match self.submitted[col].1[row] {
                    Colour::Gray => Colour::Yellow,
                    Colour::Yellow => Colour::Green,
                    Colour::Green => Colour::Gray
                };
                iced::Task::none()
            }

            Message::Clear => {
                self.submitted.clear();
                self.input.clear();
                iced::Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {

        let mut game: WordleGame = WordleGame::default();
        let mut column: iced::widget::Column<Message> = iced::widget::Column::new().spacing(20);
        for (column_idx, submission) in self.submitted.iter().enumerate() {
            let _ = game.add_information(submission.0, submission.1);
            column = column.push({
                let mut row = iced::widget::Row::new().spacing(20);
                for idx in 0..5 {
                    let c = submission.0[idx];
                    let colour = submission.1[idx].iced();

                    row = row.push(
                        iced::widget::Button::new(
                            iced::widget::text(c)
                                .width(iced::Length::FillPortion(1))
                                .color(iced::Color::from_rgb8(255, 255, 255))
                                .center()
                        )
                            .padding(10)
                            .style(move |_,_| iced::widget::button::Style {
                                text_color: iced::Color::from_rgb8(255, 255, 255),
                                background: Some(iced::Background::Color(colour)),
                                border: iced::Border::default(),
                                shadow: iced::Shadow::default()
                            })
                            .on_press(Message::Toggle(idx, column_idx))
                    )
                }
                row
            });
        }

        column
            .push({
                let mut row = iced::widget::Row::new().spacing(20);
                for idx in 0..5 {
                    let c = match self.input.chars().nth(idx) {
                        Some(c) => c,
                        None => ' '
                    };
                    row = row.push(
                        iced::widget::Button::new(
                            iced::widget::text(c)
                                .width(iced::Length::FillPortion(1))
                                .color(iced::Color::from_rgb8(255, 255, 255))
                                .center()
                        )
                            .padding(10)
                            .style(move |_,_| iced::widget::button::Style {
                                text_color: iced::Color::from_rgb8(255, 255, 255),
                                background: Some(iced::Background::Color(Colour::Gray.iced())),
                                border: iced::Border::default(),
                                shadow: iced::Shadow::default()
                            })
                    )
                }
                row
            })
            .push(
                iced::widget::text(game.recommend())
            )
            .push(
                iced::widget::Scrollable::new({
                    let mut column = iced::widget::Column::new().spacing(10);
                    for word in game.calculate() {
                        column = column.push(
                            iced::widget::text(word).width(iced::Length::Fill)
                        )
                    }
                    column
                })
            )
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::on_key_press(|key, _| match key {
            iced::keyboard::Key::Named(k) => {
                match k {
                    iced::keyboard::key::Named::Backspace => Some(Message::BackspaceInput),
                    iced::keyboard::key::Named::Enter => Some(Message::AttemptSubmit),
                    iced::keyboard::key::Named::Escape => Some(Message::Clear),
                    _ => None
                }
            },
            iced::keyboard::Key::Character(c) => {
                if c.len() == 1 {
                    let c = c.chars().find(|c| c.is_ascii_alphabetic());
                    match c {
                        Some(c) => Some(Message::UpdateInput(c)),
                        None => None
                    }
                } else {
                    None
                }
            },
            _ => None
        })
    }
}
