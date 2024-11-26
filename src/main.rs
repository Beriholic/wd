mod ui;
mod app;
mod repo;

use crate::app::App;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::new().run(&mut terminal).await;
    ratatui::restore();
    app_result
}