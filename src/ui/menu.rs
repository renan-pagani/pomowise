use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, MenuItem};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Render animated theme preview as background
    app.animation
        .current_theme
        .render_background(frame, area, app.animation.frame_index);

    // Calculate center position
    let center_x = area.width / 2;
    let center_y = area.height / 2;

    // Draw semi-transparent menu panel
    let panel_width = 30u16.min(area.width.saturating_sub(4));
    let panel_height = 12u16.min(area.height.saturating_sub(4));
    let panel_x = center_x.saturating_sub(panel_width / 2);
    let panel_y = center_y.saturating_sub(panel_height / 2);

    let panel_area = Rect::new(
        panel_x,
        panel_y,
        panel_width.min(area.width.saturating_sub(panel_x)),
        panel_height.min(area.height.saturating_sub(panel_y)),
    );

    // Draw panel background with theme-colored border
    let primary = app.animation.current_theme.primary_color();
    let bg_color = Color::Rgb(15, 15, 25);

    let panel = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(primary))
        .style(Style::default().bg(bg_color));
    frame.render_widget(panel, panel_area);

    // Draw title centered in panel
    let title = "tui-ansy-pomo";
    let title_x = panel_x + (panel_width.saturating_sub(title.len() as u16)) / 2;
    let title_y = panel_y + 2;
    if title_y < area.height && title_x < area.width {
        let title_width = (title.len() as u16).min(area.width.saturating_sub(title_x));
        frame.render_widget(
            Paragraph::new(title).style(Style::default().fg(primary).bold()),
            Rect::new(title_x, title_y, title_width, 1),
        );
    }

    // Draw theme preview label
    let theme_label = format!("Theme: {}", app.animation.current_theme.name());
    let theme_x = panel_x + (panel_width.saturating_sub(theme_label.len() as u16)) / 2;
    let theme_y = panel_y + 4;
    if theme_y < area.height && theme_x < area.width {
        let theme_width = (theme_label.len() as u16).min(area.width.saturating_sub(theme_x));
        frame.render_widget(
            Paragraph::new(theme_label).style(Style::default().fg(Color::DarkGray)),
            Rect::new(theme_x, theme_y, theme_width, 1),
        );
    }

    // Draw menu options
    let menu_y = panel_y + 6;

    let start_style = if app.menu_selection == MenuItem::Start {
        Style::default().fg(primary).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let quit_style = if app.menu_selection == MenuItem::Quit {
        Style::default().fg(primary).bold()
    } else {
        Style::default().fg(Color::White)
    };

    let start_prefix = if app.menu_selection == MenuItem::Start {
        "> "
    } else {
        "  "
    };
    let quit_prefix = if app.menu_selection == MenuItem::Quit {
        "> "
    } else {
        "  "
    };

    let start_text = format!("{}Start Pomodoro", start_prefix);
    let quit_text = format!("{}Quit", quit_prefix);

    let start_x = panel_x + (panel_width.saturating_sub(start_text.len() as u16)) / 2;
    let quit_x = panel_x + (panel_width.saturating_sub(quit_text.len() as u16)) / 2;

    if menu_y < area.height && start_x < area.width {
        let width = (start_text.len() as u16).min(area.width.saturating_sub(start_x));
        frame.render_widget(
            Paragraph::new(start_text).style(start_style),
            Rect::new(start_x, menu_y, width, 1),
        );
    }
    if menu_y + 1 < area.height && quit_x < area.width {
        let width = (quit_text.len() as u16).min(area.width.saturating_sub(quit_x));
        frame.render_widget(
            Paragraph::new(quit_text).style(quit_style),
            Rect::new(quit_x, menu_y + 1, width, 1),
        );
    }

    // Draw controls hint at bottom of panel
    let hint = "↑↓ Navigate  Enter Select";
    let hint_x = panel_x + (panel_width.saturating_sub(hint.len() as u16)) / 2;
    let hint_y = panel_y + panel_height.saturating_sub(2);
    if hint_y < area.height && hint_x < area.width {
        let hint_width = (hint.len() as u16).min(area.width.saturating_sub(hint_x));
        frame.render_widget(
            Paragraph::new(hint).style(Style::default().fg(Color::DarkGray)),
            Rect::new(hint_x, hint_y, hint_width, 1),
        );
    }
}
