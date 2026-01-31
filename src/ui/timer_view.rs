use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph},
};

use crate::animation::digits;
use crate::animation::themes::ThemeType;
use crate::app::App;
use crate::scaling::ScalingContext;

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Render the animated theme background
    app.animation
        .current_theme
        .render_background(frame, area, app.animation.frame_index);

    // Calculate timer area using scaling context
    let timer_area = centered_timer_area(area, &app.scaling, app.animation.current_font);

    // Render big digits
    let time_secs = app.timer.remaining.as_secs();
    let minutes = (time_secs / 60) as u8;
    let seconds = (time_secs % 60) as u8;

    digits::render_time_with_font(
        frame,
        timer_area,
        minutes,
        seconds,
        app.animation.current_theme.primary_color(),
        app.animation.current_theme.secondary_color(),
        app.animation.current_font,
    );

    // Draw timer overlay info (respects scaling context)
    draw_timer_overlay(frame, area, app);

    // Draw theme selector if open
    if app.theme_selector_open {
        draw_theme_selector(frame, area, app);
    }
}

/// Calculate a centered area for the timer digits based on current font
fn centered_timer_area(area: Rect, scaling: &ScalingContext, font: crate::animation::DigitFont) -> Rect {
    // Calculate actual size needed for current font
    let font_width = font.width();
    let font_height = font.height();
    let colon_width = font.colon_width();

    // Timer needs: 4 digits + colon + padding
    let timer_width = (font_width * 4 + colon_width + 4).min(area.width);
    let timer_height = (font_height + 2).min(area.height);

    // Position: centered horizontally, slightly above center vertically
    let x = area.x + area.width.saturating_sub(timer_width) / 2;
    let y = scaling.timer_y().min(area.height.saturating_sub(timer_height));

    Rect::new(
        x,
        y,
        timer_width,
        timer_height,
    )
}

fn draw_timer_overlay(frame: &mut Frame, area: Rect, app: &App) {
    // Early exit for very small terminals
    if area.width < 20 || area.height < 10 {
        return;
    }

    let scaling = &app.scaling;
    let theme = &app.animation.current_theme;
    let primary = theme.primary_color();
    let bg_color = Color::Rgb(10, 10, 20);
    let progress = app.timer.session_progress();

    // In compact mode, skip some UI elements
    let show_session_info = scaling.show_session_info;
    let show_hints = scaling.show_hints;

    // ZEN MODE: When hints are hidden, only show minimal discrete progress
    if !app.hints_visible {
        // Ultra-discrete progress line at very bottom (1px tall, no border)
        let filled_width = (area.width as f64 * progress) as u16;

        // Very subtle progress indicator - just a thin line
        let dim_primary = match primary {
            Color::Rgb(r, g, b) => Color::Rgb(r / 3, g / 3, b / 3),
            _ => Color::Rgb(40, 40, 50),
        };

        // Draw filled portion
        for x in 0..filled_width {
            frame.render_widget(
                Paragraph::new("▁").style(Style::default().fg(dim_primary)),
                Rect::new(area.x + x, area.y + area.height - 1, 1, 1),
            );
        }

        // Flash message when first hidden
        if app.hint_flash_frames > 0 {
            let flash = "h: show UI";
            let flash_len = flash.len() as u16;
            let flash_x = area.width.saturating_sub(flash_len) / 2;
            let intensity = (app.hint_flash_frames as u8 * 5).min(100);
            frame.render_widget(
                Paragraph::new(flash).style(Style::default().fg(Color::Rgb(intensity, intensity, intensity + 20))),
                Rect::new(flash_x, area.height / 2 + 8, flash_len, 1),
            );
        }

        return; // Exit early - zen mode shows nothing else
    }

    // NORMAL MODE: Full UI with all info panels

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

    // Session info in top-left (hidden in compact mode)
    let info_width = if show_session_info {
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

        info_width
    } else {
        0
    };

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

    // Progress bar at bottom (full style with border)
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

    // Auto-rotate indicator (when disabled)
    if !app.auto_rotate {
        let lock_text = "[theme locked]";
        let lock_x = area.width.saturating_sub(lock_text.len() as u16 + 2);
        if lock_x > 0 {
            frame.render_widget(
                Paragraph::new(lock_text).style(Style::default().fg(Color::Rgb(100, 80, 80))),
                Rect::new(lock_x, 3, lock_text.len() as u16, 1),
            );
        }
    }

    // Controls hint (hidden in compact mode or when scaling says to hide)
    if show_hints {
        let hint_y = area.height.saturating_sub(4);
        if hint_y > 3 {
            // Shorter hint for smaller terminals
            let hint = if area.width < 70 {
                "Space:Pause r:Reset t:Theme h:Zen q:Menu"
            } else {
                "Space: Pause  r: Reset  Tab: Skip  t: Themes  f: Font  a: Auto  h: Zen  q: Menu"
            };
            let hint_len = hint.len() as u16;
            let hint_x = area.width.saturating_sub(hint_len) / 2;
            let hint_width = hint_len.min(area.width.saturating_sub(hint_x));
            frame.render_widget(
                Paragraph::new(hint).style(Style::default().fg(Color::Rgb(80, 80, 100))),
                Rect::new(hint_x, hint_y, hint_width, 1),
            );
        }
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
