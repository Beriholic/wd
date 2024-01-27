use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::tui::Tui;

mod word;
mod tui;
fn main() ->Result<()>{
    let backend=CrosstermBackend::new(std::io::stderr());
    let terminal=Terminal::new(backend)?;

    let mut tui=Tui::new(terminal);
    tui.run()?;
    
    Ok(())
}
