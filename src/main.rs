use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::tui::Tui;

mod db;
mod tui;
mod word;
fn main() -> Result<()> {
    
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal);
    tui.start()?;
    tui.run()?;
    tui.exit()?;

    Ok(())
}
