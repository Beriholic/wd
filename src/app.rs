use crate::app::InputMode::Normal;
use crate::repo::model::Word;
use crate::repo::query::query_word;
use crate::ui;
use color_eyre::Result;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub enum InputMode {
    Normal,
    Insert,
}

pub struct App {
    pub word: Option<Word>,
    pub word_input: String,
    pub input_cursor_pos: usize,
    pub input_mode: InputMode,
    pub exit: bool,
}


impl App {
    pub fn new() -> Self {
        Self {
            word: None,
            word_input: String::new(),
            input_mode: Normal,
            input_cursor_pos: 0,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().await?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        ui::draw_ui(frame, self)
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn enter_insert_mode(&mut self) {
        match self.input_mode {
            Normal => {
                self.input_mode = InputMode::Insert;
            }
            _ => {}
        }
    }
    fn enter_normal_mode(&mut self) {
        match self.input_mode {
            InputMode::Insert => {
                self.input_mode = Normal;
            }
            _ => {}
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.input_cursor_pos.saturating_sub(1);
        self.input_cursor_pos = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.input_cursor_pos.saturating_add(1);
        self.input_cursor_pos = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.word_input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.word_input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.input_cursor_pos)
            .unwrap_or(self.word_input.len())
    }

    pub fn delete_char(&mut self) {
        if self.input_cursor_pos != 0 {
            let current_idx = self.input_cursor_pos;
            let from_left_to_current_idx = current_idx - 1;

            let before_char_to_delete = self.word_input.chars().take(from_left_to_current_idx);

            let after_char_to_delete = self.word_input.chars().skip(current_idx);

            self.word_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.word_input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.input_cursor_pos = 0;
    }

    fn reset_input(&mut self) {
        self.word_input.clear();
        self.reset_cursor();
    }


    async fn handle_events(&mut self) -> Result<()> {
        // match event::read()? {
        if let event::Event::Key(key) = event::read()? {
            match self.input_mode {
                Normal => match key.code {
                    KeyCode::Char('i') => {
                        self.enter_insert_mode()
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.exit()
                    }
                    _ => {}
                }
                InputMode::Insert  if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Char(to_insert) => match self.input_mode {
                        InputMode::Insert => self.enter_char(to_insert),
                        _ => {}
                    }
                    KeyCode::Backspace => {
                        self.delete_char();
                    }
                    KeyCode::Left => {
                        self.move_cursor_left();
                    }
                    KeyCode::Right => {
                        self.move_cursor_right();
                    }
                    KeyCode::Enter => {
                        let result = query_word(&self.word_input).await.unwrap_or(Word::default());
                        self.word = Some(result);

                        self.input_mode = Normal;
                        self.reset_input()
                    }
                    KeyCode::Esc => {
                        self.enter_normal_mode();
                    }
                    _ => (),
                },
                _ => {}
            }
        }
        Ok(())
    }
}

