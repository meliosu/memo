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

pub struct QuestionaireWidget<'a> {
    pub marker: PhantomData<&'a Questionaire>,
}

impl<'a> QuestionaireWidget<'a> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

pub struct QuestionaireWidgetState<'a> {
    pub questionaire: &'a Questionaire,
    pub question_state: Option<QuestionWidgetState<'a>>,
    pub pos: usize,
    pub score: f32,
    pub highlight: bool,
}

impl<'a> StatefulWidget for QuestionaireWidget<'a> {
    type State = QuestionaireWidgetState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if let Some(ref mut question_state) = state.question_state {
            let question = QuestionWidget::new();
            question.render(area, buf, question_state);
        } else {
            Paragraph::new(format!("Finished! Your score is {}", state.score))
                .centered()
                .render(area, buf);
        }
    }
}

impl<'a> QuestionaireWidgetState<'a> {
    pub fn new(questionaire: &'a Questionaire) -> Self {
        let question_state = if let Some(question) = questionaire.questions.first() {
            Some(QuestionWidgetState::new(question))
        } else {
            None
        };

        Self {
            question_state,
            questionaire,
            pos: 0,
            score: 0.0,
            highlight: false,
        }
    }

    pub fn update(&mut self, input: Input) {
        match input {
            Input::Enter => {
                if !self.highlight {
                    self.highlight = true;

                    if let Some(ref mut state) = self.question_state {
                        state.update(Input::Enter);
                    }
                } else {
                    if self.pos < self.questionaire.questions.len() {
                        if let Some(ref state) = self.question_state {
                            self.score += state.score();
                        }

                        self.pos += 1;
                        self.highlight = false;
                    }

                    self.question_state = self
                        .questionaire
                        .questions
                        .get(self.pos)
                        .map(QuestionWidgetState::new);
                }
            }

            other => {
                if let Some(ref mut state) = self.question_state {
                    state.update(other);
                }
            }
        }
    }
}

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

    pub fn score(&self) -> f32 {
        match &self.question.answers {
            Answers::Text(answers) => {
                if answers
                    .iter()
                    .any(|answer| answer.to_lowercase() == self.text.to_lowercase())
                {
                    1.0
                } else {
                    0.0
                }
            }

            Answers::Choice((_, correct)) => {
                if self.picked.contains(&correct) {
                    1.0
                } else {
                    0.0
                }
            }

            Answers::Multi(answers) => {
                self.picked.iter().filter(|&&pick| answers[pick].1).count() as f32
                    / answers.len() as f32
            }
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

        let area = center_rect(area.width / 3, area.height * 2 / 3, area);

        let question_block = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .style(Style::new().magenta());

        let question = Paragraph::new(state.question.problem.clone()).block(question_block);

        match &state.question.answers {
            Answers::Text(answer) => {
                let color = if !state.highlight_correct {
                    Color::Black
                } else if state.highlight_correct
                    && answer
                        .iter()
                        .any(|answer| answer.to_lowercase() == state.text.to_lowercase())
                {
                    Color::Green
                } else {
                    Color::Red
                };

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
