use std::{iter, marker::PhantomData};

use ratatui::style::Color;
use ratatui::widgets::Paragraph;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders},
};

use crate::types::*;

use super::input::Input;
use super::utils::center_rect;

pub struct QuestionWidget<'a> {
    pub marker: PhantomData<&'a Question>,
}

pub struct QuestionWidgetState<'a> {
    pub question: &'a Question,
    pub picked: Vec<usize>,
    pub text: String,
    pub highlight_correct: bool,
}

impl<'a> QuestionWidget<'a> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<'a> QuestionWidgetState<'a> {
    pub fn new(question: &'a Question) -> Self {
        Self {
            question,
            picked: Vec::new(),
            text: String::new(),
            highlight_correct: false,
        }
    }

    pub fn update(&mut self, input: Input) {
        match self.question.answers {
            Answers::Text(_) => match input {
                Input::Enter => self.highlight_correct = true,
                Input::Char(c) => self.text.push(c),
                Input::Back => {
                    self.text.pop();
                }

                _ => {}
            },

            Answers::Choice((ref answers, _)) => match input {
                Input::Enter => self.highlight_correct = true,
                Input::Pick(pick) => {
                    if pick < answers.len() {
                        self.picked = vec![pick];
                    }
                }

                _ => {}
            },

            Answers::Multi(ref answers) => match input {
                Input::Enter => self.highlight_correct = true,
                Input::Pick(pick) => {
                    if pick < answers.len() {
                        if let Some(pos) = self.picked.iter().position(|p| *p == pick) {
                            self.picked.remove(pos);
                        } else {
                            self.picked.push(pick);
                        }
                    }
                }

                _ => {}
            },
        }
    }
}

impl<'a> StatefulWidget for QuestionWidget<'a> {
    type State = QuestionWidgetState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let answer_color = |picked: bool, correct: bool, highlight: bool| {
            if picked {
                if !highlight {
                    Color::Blue
                } else {
                    if correct {
                        Color::Green
                    } else {
                        Color::Red
                    }
                }
            } else {
                if !highlight {
                    Color::Black
                } else {
                    if correct {
                        Color::Yellow
                    } else {
                        Color::Black
                    }
                }
            }
        };

        let area = center_rect(area.width / 3, area.height / 2, area);

        let question_block = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .style(Style::new().magenta());

        let question = Paragraph::new(state.question.problem.clone()).block(question_block);

        match &state.question.answers {
            Answers::Text(answer) => {
                let color = answer_color(true, *answer == state.text, state.highlight_correct);

                let style = Style {
                    bg: Some(color),
                    ..Default::default()
                };

                let answer = Paragraph::new(state.text.clone())
                    .style(style)
                    .block(Block::new().borders(Borders::all()).style(style));

                let [q_area, a_area] =
                    Layout::vertical([Constraint::Fill(80), Constraint::Fill(20)]).areas(area);

                question.render(q_area, buf);
                answer.render(a_area, buf);
            }

            Answers::Choice((answers, correct)) => {
                let answers: Vec<_> = answers
                    .iter()
                    .enumerate()
                    .map(|(idx, answer)| {
                        let color = answer_color(
                            state.picked.contains(&idx),
                            idx == *correct,
                            state.highlight_correct,
                        );

                        let style = Style {
                            bg: Some(color),
                            ..Default::default()
                        };

                        Paragraph::new(format!("{}. {}", idx + 1, answer))
                            .style(style)
                            .block(Block::new().borders(Borders::all()).style(style))
                    })
                    .collect();

                let constraints: Vec<_> = iter::once(Constraint::Fill(1))
                    .chain(answers.iter().map(|_| Constraint::Min(3)))
                    .collect();

                let layout = Layout::vertical(constraints).split(area);

                question.render(layout[0], buf);

                for (i, answer) in answers.into_iter().enumerate() {
                    answer.render(layout[i + 1], buf);
                }
            }

            Answers::Multi(multi) => {
                let answers: Vec<_> = multi
                    .iter()
                    .enumerate()
                    .map(|(idx, (answer, correct))| {
                        let color = answer_color(
                            state.picked.contains(&idx),
                            *correct,
                            state.highlight_correct,
                        );

                        let style = Style {
                            bg: Some(color),
                            ..Default::default()
                        };

                        Paragraph::new(format!("{}. {}", idx + 1, answer))
                            .style(style)
                            .block(Block::new().borders(Borders::all()).style(style))
                    })
                    .collect();

                let constraints: Vec<_> = iter::once(Constraint::Fill(1))
                    .chain(answers.iter().map(|_| Constraint::Min(3)))
                    .collect();

                let layout = Layout::vertical(constraints).split(area);

                question.render(layout[0], buf);

                for (i, answer) in answers.into_iter().enumerate() {
                    answer.render(layout[i + 1], buf);
                }
            }
        }
    }
}
