mod menu;
mod timer_view;
pub mod widgets;

use ratatui::prelude::*;

use crate::app::{App, AppScreen};

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        AppScreen::Menu => menu::draw(frame, app),
        AppScreen::Timer => timer_view::draw(frame, app),
    }
}
