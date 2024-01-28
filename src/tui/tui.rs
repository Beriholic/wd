use std::io;

use color_eyre::eyre::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::tui::app::App;

use super::{app::InputMod, ui::draw_ui};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Tui { terminal }
    }
    pub fn start(&mut self) -> Result<()> {
        enable_raw_mode()?;
        crossterm::execute!(io::stdout(), Hide, EnterAlternateScreen,)?;
        self.terminal.clear()?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stdout(), Show, LeaveAlternateScreen,)?;
        Ok(())
    }
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| draw_ui(f, app))?;
        Ok(())
    }
    
    pub fn run(&mut self) -> Result<()> {
        let mut app = App::new();
        loop {
            self.draw(&mut app)?;

            if let event::Event::Key(key) = event::read()? {
                match app.input_mod {
                    InputMod::Normal => match key.code {
                        KeyCode::Char('i') => {
                            app.input_mod = InputMod::Insert;
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMod::Insert if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            app.input_mod = InputMod::Normal;
                            app.submit_query();
                            app.search();
                        }
                        KeyCode::Char(to_insert) => {
                            app.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            app.delete_char();
                        }
                        KeyCode::Left => {
                            app.move_cursor_left();
                        }
                        KeyCode::Right => {
                            app.move_cursor_right();
                        }
                        KeyCode::Esc => {
                            app.input_mod = InputMod::Normal;
                        }

                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
