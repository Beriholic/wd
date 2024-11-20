use crate::app::InputMode;
use ratatui::layout::{Alignment, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{block, Block, BorderType, Borders, Paragraph, Wrap};
use ratatui::Frame;


pub fn draw_word_name(
    mut name: &str,
    frame: &mut Frame,
    layout: &Rect,
) {
    let style = match name.len() {
        0 => {
            name = "单词未找到";
            Style::default().fg(Color::Red)
        }
        _ => Style::default()
    };

    let block = rounded_border_block("Word").style(style);

    let p = Paragraph::new(name).wrap(Wrap { trim: true }).block(block);


    frame.render_widget(p, *layout);
}

pub fn draw_word_translation(
    translation: &str,
    frame: &mut Frame,
    layout: &Rect,
) {
    let p = Paragraph::new(translation).block(rounded_border_block("Translation"));

    frame.render_widget(p, *layout)
}


pub fn draw_word_definition(
    definition: &str,
    frame: &mut Frame,
    layout: &Rect,
) {
    let p = Paragraph::new(definition).block(rounded_border_block("Definition"));
    frame.render_widget(p, *layout)
}

pub fn draw_word_tags(
    tags: &str,
    frame: &mut Frame,
    layout: &Rect,
) {
    let p = Paragraph::new(tags).block(rounded_border_block("Tags"));
    frame.render_widget(p, *layout);
}

pub fn draw_word_exchange(
    exchange: &str,
    frame: &mut Frame,
    layout: &Rect,
) {
    let p = Paragraph::new(exchange).block(rounded_border_block("Exchange"));
    frame.render_widget(p, *layout);
}


pub fn draw_word_input(
    input: &str,
    cursor_index: &usize,
    input_mode: &InputMode,
    frame: &mut Frame,
    layout: &Rect,
) {
    let input = Paragraph::new(input).block(Block::bordered().title("Input"));

    frame.render_widget(input, *layout);

    match input_mode {
        InputMode::Normal => {}
        InputMode::Insert => frame.set_cursor_position(Position::new(
            layout.x + *cursor_index as u16 + 1,
            layout.y + 1,
        ))
    }
}
pub fn draw_footer(frame: &mut Frame, layout: &Rect) {
    let footer = Paragraph::new("press 'i' to insert | press 'q' or 'esc' to exit")
        .style(Style::default().italic().fg(Color::LightYellow));

    frame.render_widget(footer, *layout);
}

fn rounded_border_block(title: &str) -> Block {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(block::Title::from(title).alignment(Alignment::Left))
}
