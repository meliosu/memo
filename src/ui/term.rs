use std::io;

use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn setup() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;

    std::panic::set_hook(Box::new(|info| {
        _ = terminal::disable_raw_mode();
        _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);

        println!("{info}");
    }));

    Ok(())
}

pub fn teardown() -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;

    _ = std::panic::take_hook();

    Ok(())
}
