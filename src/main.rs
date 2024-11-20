mod ui;
mod app;
mod repo;

use color_eyre::Result;

use crate::app::App;
use ratatui::{
    style::Stylize,
    widgets::Widget,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
