mod db;
mod entity;
mod tui;
use color_eyre::eyre::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::tui::Tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal);
    tui.start()?;
    tui.run()?;
    tui.exit()?;

    Ok(())
}
