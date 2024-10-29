use ratatui::prelude::*;

pub fn center_rect(width: u16, height: u16, area: Rect) -> Rect {
    let [_, mid, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(width),
        Constraint::Fill(1),
    ])
    .areas(area);

    let [_, mid, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(height),
        Constraint::Fill(1),
    ])
    .areas(mid);

    mid
}
