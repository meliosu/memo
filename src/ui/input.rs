use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub enum Input {
    Pick(usize),
    Char(char),
    Back,
    Enter,
    Escape,
}

pub fn read() -> io::Result<Input> {
    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => return Ok(Input::Enter),

            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                ..
            }) => return Ok(Input::Back),

            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) if c.is_ascii_digit() => {
                return Ok(Input::Pick(c.to_digit(10).unwrap() as usize - 1))
            }

            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => return Ok(Input::Char(c)),

            Event::Key(KeyEvent {
                code: KeyCode::Esc, ..
            }) => return Ok(Input::Escape),

            _ => continue,
        }
    }
}
