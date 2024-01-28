use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{block, Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

use super::app::InputMod;

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(7),
            Constraint::Percentage(28),
            Constraint::Percentage(28),
            Constraint::Percentage(7),
            Constraint::Percentage(20),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_block_str(
        &format!("{} [{}]", &app.word.word, &app.word.phonetic),
        "Word",
        f,
        &layout[0],
    );
    draw_block_vec(&app.word.translation, "Translation", f, &layout[1]);
    draw_block_vec(&app.word.definition, "Definition", f, &layout[2]);
    draw_block_str(&app.word.tags, "Tag", f, &layout[3]);
    draw_block_vec(&app.word.exchanges, "Exchange", f, &layout[4]);
    draw_input_block(f, &layout[5], app);
}
fn draw_block_vec(info: &Vec<String>, tag: &str, f: &mut Frame, layout: &Rect) {
    let lines = info
        .iter()
        .map(|item| {
            let span = Span::styled(format!("  {}", item), Style::default().fg(Color::White));
            Line::from(span)
        })
        .collect::<Vec<Line>>();

    let text = Text::from(lines);
    let p = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(block::Title::from(tag).alignment(Alignment::Left)),
    );
    f.render_widget(p, *layout);
}
fn draw_block_str(str: &str, tag: &str, f: &mut Frame, layout: &Rect) {
    let span = Span::styled(format!("  {}", str), Style::default().fg(Color::White));
    let line = Line::from(span);
    let text = Text::from(line);
    let p = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(block::Title::from(tag).alignment(Alignment::Left)),
    );
    f.render_widget(p, *layout);
}
fn draw_input_block(f: &mut Frame, layout: &Rect, app: &App) {
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mod {
            InputMod::Normal => Style::default(),
            InputMod::Insert => Style::default().fg(Color::LightYellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(input, *layout);

    match app.input_mod {
        InputMod::Normal => {}
        InputMod::Insert => f.set_cursor(layout.x + app.cursor_pos as u16 + 1, layout.y + 1),
    }
}
