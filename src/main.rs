use std::io;

use memo::{
    types::Questionaire,
    ui::{
        self,
        input::Input,
        widget::{QuestionWidget, QuestionWidgetState},
    },
};
use ratatui::widgets::StatefulWidget;
use ratatui::{prelude::CrosstermBackend, Terminal};

fn main() -> io::Result<()> {
    let Some(path) = std::env::args().nth(1) else {
        panic!("missing questionaire path");
    };

    let questionaire = Questionaire::load(path).unwrap_or_else(|err| {
        panic!("error loading: {err}");
    });

    ui::term::setup()?;

    let mut term = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut state = QuestionWidgetState::new(&questionaire.questions[1]);

    loop {
        term.draw(|frame| {
            QuestionWidget::new().render(frame.area(), frame.buffer_mut(), &mut state);
        })?;

        let input = ui::input::read()?;

        match input {
            Input::Escape => break,
            other => state.update(other),
        }
    }

    ui::term::teardown()?;
    Ok(())
}
