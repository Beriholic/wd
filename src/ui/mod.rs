use crate::app::App;
use crate::repo::model::Stardict;
use crate::ui::ui::{draw_footer, draw_word_definition, draw_word_exchange, draw_word_input, draw_word_name, draw_word_tags, draw_word_translation};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::widgets::Widget;
use ratatui::Frame;

pub mod ui;


pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let word = match &app.word {
        Some(word) => word,
        None => &Stardict::not_found()
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Length(3),
            Constraint::Percentage(25),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.area());

    draw_word_name(&word.name, frame, &layout[0]);
    draw_word_translation(&word.translation, frame, &layout[1]);
    draw_word_definition(&word.definition, frame, &layout[2]);
    draw_word_tags(&word.tags, frame, &layout[3]);
    draw_word_exchange(&word.exchange, frame, &layout[4]);
    draw_word_input(&app.word_input, &app.input_cursor_pos, &app.input_mode, frame, &layout[5]);
    draw_footer(frame, &layout[6])


    // draw_word_input(&app.word_input, app.input_cursor_pos, &app.input_mode, frame, &layout[])
}

