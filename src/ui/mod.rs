mod menu;
mod timer_view;
pub mod widgets;

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::app::{App, AppScreen};
use crate::scaling::{MIN_WIDTH, MIN_HEIGHT};

pub fn draw(frame: &mut Frame, app: &App) {
    // Check if terminal is too small
    if app.scaling.is_too_small() {
        draw_too_small_warning(frame, app);
        return;
    }

    match app.screen {
        AppScreen::Menu => menu::draw(frame, app),
        AppScreen::Timer => timer_view::draw(frame, app),
    }
}

/// Draw a warning message when terminal is too small
fn draw_too_small_warning(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Clear with dark background
    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Rgb(20, 20, 30))),
        area,
    );

    let message = format!(
        "Terminal too small!\n\n\
         Current: {}x{}\n\
         Minimum: {}x{}\n\n\
         Please resize your terminal.",
        app.scaling.width, app.scaling.height,
        MIN_WIDTH, MIN_HEIGHT
    );

    let paragraph = Paragraph::new(message)
        .style(Style::default().fg(Color::Rgb(255, 100, 100)))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 50, 50)))
                .title(" ⚠ pomowise ")
                .title_style(Style::default().fg(Color::Rgb(255, 150, 50))),
        );

    // Center the message
    let width = 30.min(area.width);
    let height = 10.min(area.height);
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;

    frame.render_widget(
        paragraph,
        Rect::new(x, y, width, height),
    );
}
