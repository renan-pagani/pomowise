mod app;
mod timer;
mod notification;
mod ui;
mod animation;
mod scaling;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::{App, AppScreen};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100); // 10 FPS

    loop {
        // Draw UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Handle events with timeout for animation
        if event::poll(tick_rate)? {
            match event::read()? {
                // Handle terminal resize
                Event::Resize(width, height) => {
                    app.update_dimensions(width, height);
                }

                // Handle key events
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match app.screen {
                        AppScreen::Menu => match key.code {
                            KeyCode::Up | KeyCode::Char('k') => app.menu_up(),
                            KeyCode::Down | KeyCode::Char('j') => app.menu_down(),
                            KeyCode::Enter => {
                                if !app.menu_select() {
                                    return Ok(());
                                }
                            }
                            KeyCode::Char('q') => return Ok(()),
                            _ => {}
                        },
                        AppScreen::Timer => {
                            // Theme selector is open - handle its input
                            if app.theme_selector_open {
                                match key.code {
                                    KeyCode::Up | KeyCode::Char('k') => app.theme_selector_up(),
                                    KeyCode::Down | KeyCode::Char('j') => app.theme_selector_down(),
                                    KeyCode::Enter => app.theme_selector_confirm(),
                                    KeyCode::Esc | KeyCode::Char('T') => app.theme_selector_cancel(),
                                    _ => {}
                                }
                            } else {
                                // Normal timer controls
                                match key.code {
                                    KeyCode::Char(' ') => app.toggle_pause(),
                                    KeyCode::Char('r') => app.reset_session(),
                                    KeyCode::Char('q') => app.quit_to_menu(),
                                    KeyCode::Tab => app.skip_to_next(),
                                    KeyCode::Char('T') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                                        app.toggle_theme_selector();
                                    }
                                    KeyCode::Char('t') => {
                                        // Also allow lowercase 't' for convenience
                                        app.toggle_theme_selector();
                                    }
                                    KeyCode::Char('f') => {
                                        // Cycle through font styles (disables adaptive mode)
                                        app.adaptive_font = false;
                                        app.animation.next_font();
                                    }
                                    KeyCode::Char('F') if key.modifiers.contains(KeyModifiers::SHIFT) => {
                                        // Toggle adaptive font mode
                                        app.toggle_adaptive_font();
                                    }
                                    KeyCode::Char('a') => {
                                        // Toggle auto-rotation
                                        app.toggle_auto_rotate();
                                    }
                                    KeyCode::Char('h') => {
                                        // Toggle hints visibility
                                        app.toggle_hints();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                _ => {} // Ignore other events (mouse, focus, etc.)
            }
        }

        // Update timer and animation
        app.tick();

        if app.should_quit {
            return Ok(());
        }
    }
}
