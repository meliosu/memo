use std::io;

use memo::{
    types::Questionaire,
    ui::{
        self,
        input::Input,
        widget::{QuestionaireWidget, QuestionaireWidgetState},
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

    let mut state = QuestionaireWidgetState::new(&questionaire);

    loop {
        term.draw(|frame| {
            QuestionaireWidget::new().render(frame.area(), frame.buffer_mut(), &mut state);
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
