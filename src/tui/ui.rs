use crate::tui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block, Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

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
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_block_word(&app.word.word_info, f, &layout[0]);
    draw_block_vec(&app.word.translation, "Translation", f, &layout[1]);
    draw_block_vec(&app.word.definition, "Definition", f, &layout[2]);
    draw_block_str(&app.word.tags, "Tag", f, &layout[3]);
    draw_block_vec(&app.word.exchanges, "Exchange", f, &layout[4]);
    draw_input_block(f, &layout[5], app);
    draw_footer(f, &layout[6]);
}

fn draw_block_word(info: &str, f: &mut Frame, lazyout: &Rect) {
    let style = match info {
        "?" => Style::default().fg(Color::Red),
        _ => Style::default(),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(style)
        .title(block::Title::from("Word").alignment(Alignment::Left));

    let text = match info {
        "?" => Text::from(Line::from(Span::styled(
            "  单词未找到",
            Style::default().fg(Color::Red),
        ))),
        _ => Text::from(Line::from(Span::styled(
            format!("  {}", info),
            Style::default().fg(Color::White),
        ))),
    };

    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(block);
    f.render_widget(p, *lazyout);
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
    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(block::Title::from(tag)),
    );

    f.render_widget(p, *layout);
}
fn draw_block_str(info: &str, tag: &str, f: &mut Frame, layout: &Rect) {
    let text = Span::styled(format!("  {}", info), Style::default().fg(Color::White));
    let text = Text::from(Line::from(text));
    let p = Paragraph::new(text).wrap(Wrap { trim: true }).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default())
            .title(block::Title::from(tag)),
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
fn draw_footer(f: &mut Frame, layout: &Rect) {
    let footer = Paragraph::new("press 'i' to insert | press 'q' or 'esc' to exit")
        .style(Style::default().italic().fg(Color::LightYellow));

    f.render_widget(footer, *layout);
}
