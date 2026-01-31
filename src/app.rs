use crate::animation::AnimationEngine;
use crate::animation::themes::ThemeType;
use crate::notification::notify_session_end;
use crate::timer::{PomodoroTimer, TimerState};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppScreen {
    Menu,
    Timer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem {
    Start,
    Quit,
}

pub struct App {
    pub screen: AppScreen,
    pub menu_selection: MenuItem,
    pub timer: PomodoroTimer,
    pub animation: AnimationEngine,
    pub should_quit: bool,
    pub theme_selector_open: bool,
    pub theme_selector_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::Menu,
            menu_selection: MenuItem::Start,
            timer: PomodoroTimer::new(),
            animation: AnimationEngine::new(),
            should_quit: false,
            theme_selector_open: false,
            theme_selector_index: 0,
        }
    }

    pub fn menu_up(&mut self) {
        self.menu_selection = MenuItem::Start;
    }

    pub fn menu_down(&mut self) {
        self.menu_selection = MenuItem::Quit;
    }

    /// Returns false if app should quit
    pub fn menu_select(&mut self) -> bool {
        match self.menu_selection {
            MenuItem::Start => {
                self.screen = AppScreen::Timer;
                self.timer.start();
                self.animation.reset();
                true
            }
            MenuItem::Quit => false,
        }
    }

    pub fn toggle_pause(&mut self) {
        self.timer.toggle_pause();
    }

    pub fn reset_session(&mut self) {
        self.timer.reset_current_session();
        self.animation.reset();
    }

    pub fn quit_to_menu(&mut self) {
        self.screen = AppScreen::Menu;
        self.timer = PomodoroTimer::new();
        self.animation.reset();
    }

    /// Skip to next interval/cycle AND change theme (Tab key)
    pub fn skip_to_next(&mut self) {
        self.timer.advance_state();
        self.animation.rotate_theme();
    }

    /// Toggle theme selector overlay (Shift+T)
    pub fn toggle_theme_selector(&mut self) {
        self.theme_selector_open = !self.theme_selector_open;
        if self.theme_selector_open {
            // Set selector to current theme
            let themes = ThemeType::all();
            self.theme_selector_index = themes
                .iter()
                .position(|&t| t == self.animation.current_theme)
                .unwrap_or(0);
        }
    }

    /// Navigate theme selector up
    pub fn theme_selector_up(&mut self) {
        let themes = ThemeType::all();
        if self.theme_selector_index > 0 {
            self.theme_selector_index -= 1;
        } else {
            self.theme_selector_index = themes.len() - 1;
        }
        // Preview the theme as we navigate
        self.animation.set_theme(themes[self.theme_selector_index]);
    }

    /// Navigate theme selector down
    pub fn theme_selector_down(&mut self) {
        let themes = ThemeType::all();
        self.theme_selector_index = (self.theme_selector_index + 1) % themes.len();
        // Preview the theme as we navigate
        self.animation.set_theme(themes[self.theme_selector_index]);
    }

    /// Confirm theme selection
    pub fn theme_selector_confirm(&mut self) {
        let themes = ThemeType::all();
        self.animation.set_theme(themes[self.theme_selector_index]);
        self.theme_selector_open = false;
    }

    /// Cancel theme selection (restore previous)
    pub fn theme_selector_cancel(&mut self) {
        self.theme_selector_open = false;
        // Theme already set during navigation, just close
    }

    pub fn tick(&mut self) {
        // Always tick animation (for menu preview too)
        self.animation.tick(&self.timer.state);

        if self.screen == AppScreen::Timer {
            let previous_state = self.timer.state.clone();
            self.timer.tick();

            // Check for state transition to send notification
            if !matches!(self.timer.state, TimerState::Idle)
                && !matches!(self.timer.state, TimerState::Paused(_))
                && std::mem::discriminant(&previous_state)
                    != std::mem::discriminant(&self.timer.state)
            {
                let msg = match previous_state {
                    TimerState::Work { .. } => Some("Work session"),
                    TimerState::ShortBreak { .. } => Some("Short break"),
                    TimerState::LongBreak => Some("Long break"),
                    _ => None,
                };
                if let Some(session_type) = msg {
                    notify_session_end(session_type);
                }
            }
        }
    }
}
