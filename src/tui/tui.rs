use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::tui::app::App;

use super::ui::draw_ui;

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Tui { terminal }
    }
    pub fn start(&mut self) -> Result<()> {
        enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen,)?;
        self.terminal.clear()?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen,)?;
        Ok(())
    }
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| draw_ui(f, app))?;
        Ok(())
    }
    
    pub fn run(&mut self) -> Result<()> {
        let mut app = App::new();
        app.search();

        self.start()?;

        loop {
            self.draw(&mut app)?;
            if event::poll(std::time::Duration::from_millis(250))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        self.exit()?;
        Ok(())
    }
}
