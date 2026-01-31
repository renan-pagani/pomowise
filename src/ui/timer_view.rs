use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::animation::digits;
use crate::animation::themes::ThemeType;
use crate::app::App;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Render the animated theme background
    app.animation
        .current_theme
        .render_background(frame, area, app.animation.frame_index);

    // Calculate timer area (centered region for digits)
    let timer_area = centered_timer_area(area);

    // Render big digits
    let time_secs = app.timer.remaining.as_secs();
    let minutes = (time_secs / 60) as u8;
    let seconds = (time_secs % 60) as u8;

    digits::render_time(
        frame,
        timer_area,
        minutes,
        seconds,
        app.animation.current_theme.primary_color(),
        app.animation.current_theme.secondary_color(),
    );

    // Draw timer overlay info
    draw_timer_overlay(frame, area, app);

    // Draw theme selector if open
    if app.theme_selector_open {
        draw_theme_selector(frame, area, app);
    }
}

/// Calculate a centered area for the timer digits
fn centered_timer_area(area: Rect) -> Rect {
    let (timer_width, timer_height) = digits::timer_dimensions();

    // Add some padding
    let padded_width = timer_width + 4;
    let padded_height = timer_height + 2;

    let x = area.x + area.width.saturating_sub(padded_width) / 2;
    let y = area.y + area.height.saturating_sub(padded_height) / 2;

    Rect::new(
        x,
        y,
        padded_width.min(area.width.saturating_sub(x)),
        padded_height.min(area.height.saturating_sub(y)),
    )
}

fn draw_timer_overlay(frame: &mut Frame, area: Rect, app: &App) {
    if area.width < 20 || area.height < 10 {
        return;
    }

    let theme = &app.animation.current_theme;
    let primary = theme.primary_color();
    let bg_color = Color::Rgb(10, 10, 20);

    // Timer display in top-right (small digital clock)
    let time_secs = app.timer.remaining.as_secs();
    let minutes = time_secs / 60;
    let seconds = time_secs % 60;
    let time_str = format!("{:02}:{:02}", minutes, seconds);

    let time_x = area.width.saturating_sub(11);

    let timer_bg = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(primary))
        .style(Style::default().bg(bg_color));
    let timer_box_width = 10.min(area.width.saturating_sub(time_x.saturating_sub(1)));
    frame.render_widget(
        timer_bg,
        Rect::new(time_x.saturating_sub(1), 0, timer_box_width, 3),
    );

    frame.render_widget(
        Paragraph::new(time_str)
            .style(Style::default().fg(primary).bold())
            .alignment(Alignment::Center),
        Rect::new(time_x, 1, 8.min(area.width.saturating_sub(time_x)), 1),
    );

    // Session info in top-left
    let session_name = app.timer.session_name();
    let lap_info = if app.timer.total_laps() > 0 {
        format!(" (Lap {}/{})", app.timer.current_lap(), app.timer.total_laps())
    } else {
        String::new()
    };
    let session_str = format!("{}{}", session_name, lap_info);

    let info_width = (session_str.len() as u16 + 4).min(area.width);
    let info_bg = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(primary))
        .style(Style::default().bg(bg_color));
    frame.render_widget(info_bg, Rect::new(0, 0, info_width, 3));

    frame.render_widget(
        Paragraph::new(session_str).style(Style::default().fg(primary)),
        Rect::new(2, 1, info_width.saturating_sub(4), 1),
    );

    // Theme name indicator (top center)
    let theme_name = format!(" {} ", theme.name());
    let theme_width = theme_name.len() as u16 + 2;
    let theme_x = area.width.saturating_sub(theme_width) / 2;
    if theme_x > info_width && theme_x + theme_width < time_x.saturating_sub(1) {
        frame.render_widget(
            Paragraph::new(theme_name)
                .style(Style::default().fg(Color::DarkGray).bg(bg_color)),
            Rect::new(theme_x, 0, theme_width, 1),
        );
    }

    // Progress bar at bottom
    let progress = app.timer.session_progress();
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(primary))
                .style(Style::default().bg(bg_color)),
        )
        .gauge_style(
            Style::default()
                .fg(primary)
                .bg(theme.secondary_color()),
        )
        .ratio(progress);
    frame.render_widget(
        gauge,
        Rect::new(0, area.height.saturating_sub(3), area.width, 3.min(area.height)),
    );

    // Controls hint
    let hint = "Space: Pause  r: Reset  Tab: Skip  t: Themes  q: Menu";
    let hint_len = hint.len() as u16;
    let hint_x = area.width.saturating_sub(hint_len) / 2;
    let hint_y = area.height.saturating_sub(4);
    let hint_width = hint_len.min(area.width.saturating_sub(hint_x));
    if hint_y > 3 {
        frame.render_widget(
            Paragraph::new(hint).style(Style::default().fg(Color::Rgb(80, 80, 100))),
            Rect::new(hint_x, hint_y, hint_width, 1),
        );
    }
}

fn draw_theme_selector(frame: &mut Frame, area: Rect, app: &App) {
    let themes = ThemeType::all();
    let primary = app.animation.current_theme.primary_color();
    let bg_color = Color::Rgb(15, 15, 25);

    // Panel dimensions
    let panel_width = 24u16.min(area.width.saturating_sub(4));
    let panel_height = (themes.len() as u16 + 4).min(area.height.saturating_sub(4));

    // Position on the right side of screen
    let panel_x = area.width.saturating_sub(panel_width + 2);
    let panel_y = (area.height.saturating_sub(panel_height)) / 2;

    let panel_area = Rect::new(
        panel_x,
        panel_y,
        panel_width.min(area.width.saturating_sub(panel_x)),
        panel_height.min(area.height.saturating_sub(panel_y)),
    );

    // Draw panel background
    let panel = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(primary))
        .title(" Themes ")
        .title_style(Style::default().fg(primary).bold())
        .title_bottom(" ↑↓ Enter Esc ")
        .style(Style::default().bg(bg_color));
    frame.render_widget(panel, panel_area);

    // Draw theme list
    for (i, theme) in themes.iter().enumerate() {
        let y = panel_y + 2 + i as u16;
        if y >= panel_y + panel_height - 1 {
            break;
        }

        let is_selected = i == app.theme_selector_index;
        let prefix = if is_selected { "▶ " } else { "  " };
        let text = format!("{}{}", prefix, theme.name());

        let style = if is_selected {
            Style::default().fg(primary).bold()
        } else {
            Style::default().fg(Color::White)
        };

        let text_x = panel_x + 2;
        let text_width = (text.len() as u16).min(panel_width.saturating_sub(4));

        if text_x < area.width && y < area.height {
            frame.render_widget(
                Paragraph::new(text).style(style),
                Rect::new(text_x, y, text_width, 1),
            );
        }
    }

}
