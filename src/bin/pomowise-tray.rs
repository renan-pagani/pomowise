use std::time::Duration;
use std::thread;
use std::process::Command;

use tray_icon::{
    TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
    Icon,
};

use pomowise::ipc;
use pomowise::timer::{TimerSnapshot, TimerState};

#[cfg(target_os = "linux")]
fn init_event_loop() {
    gtk::init().expect("Failed to init GTK");
}

#[cfg(not(target_os = "linux"))]
fn init_event_loop() {}

#[cfg(target_os = "linux")]
fn process_events() {
    while gtk::events_pending() {
        gtk::main_iteration_do(false);
    }
}

#[cfg(not(target_os = "linux"))]
fn process_events() {}

// 3x5 pixel font for digits 0-9 and colon
const DIGIT_FONT: [[u8; 5]; 11] = [
    [0b111, 0b101, 0b101, 0b101, 0b111], // 0
    [0b010, 0b110, 0b010, 0b010, 0b111], // 1
    [0b111, 0b001, 0b111, 0b100, 0b111], // 2
    [0b111, 0b001, 0b111, 0b001, 0b111], // 3
    [0b101, 0b101, 0b111, 0b001, 0b001], // 4
    [0b111, 0b100, 0b111, 0b001, 0b111], // 5
    [0b111, 0b100, 0b111, 0b101, 0b111], // 6
    [0b111, 0b001, 0b010, 0b010, 0b010], // 7
    [0b111, 0b101, 0b111, 0b101, 0b111], // 8
    [0b111, 0b101, 0b111, 0b001, 0b111], // 9
    [0b000, 0b010, 0b000, 0b010, 0b000], // : (index 10)
];

fn state_color(snapshot: &TimerSnapshot) -> (u8, u8, u8) {
    if snapshot.is_paused {
        (255, 165, 0)
    } else {
        match &snapshot.state {
            TimerState::Work { .. } => (220, 50, 50),
            TimerState::ShortBreak { .. } => (50, 180, 50),
            TimerState::LongBreak => (50, 100, 220),
            _ => (128, 128, 128),
        }
    }
}

fn create_icon_with_time(bg: (u8, u8, u8), mins: u64, secs: u64) -> Icon {
    let size = 64u32;
    let scale = 3u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    // Fill background
    for i in 0..(size * size) as usize {
        rgba[i * 4] = bg.0;
        rgba[i * 4 + 1] = bg.1;
        rgba[i * 4 + 2] = bg.2;
        rgba[i * 4 + 3] = 255;
    }

    // "MM:SS" = 5 glyphs, each 3*scale wide with scale gap
    // Total width: 5*(3*3) + 4*3 = 45+12 = 57px, centered in 64 => offset_x = 3
    // Height: 5*3 = 15px, centered in 64 => offset_y = 24
    let glyphs = [
        (mins / 10) as usize,
        (mins % 10) as usize,
        10, // colon
        (secs / 10) as usize,
        (secs % 10) as usize,
    ];

    let offset_x: u32 = 3;
    let offset_y: u32 = 24;

    for (gi, &glyph) in glyphs.iter().enumerate() {
        let gx = offset_x + (gi as u32) * (3 * scale + scale);
        for row in 0..5u32 {
            let bits = DIGIT_FONT[glyph][row as usize];
            for col in 0..3u32 {
                if bits & (1 << (2 - col)) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let px = gx + col * scale + sx;
                            let py = offset_y + row * scale + sy;
                            if px < size && py < size {
                                let idx = ((py * size + px) * 4) as usize;
                                rgba[idx] = 255;
                                rgba[idx + 1] = 255;
                                rgba[idx + 2] = 255;
                                rgba[idx + 3] = 255;
                            }
                        }
                    }
                }
            }
        }
    }

    Icon::from_rgba(rgba, size, size).expect("Failed to create icon")
}

fn format_tooltip(snapshot: &TimerSnapshot) -> String {
    let mins = snapshot.remaining_secs / 60;
    let secs = snapshot.remaining_secs % 60;
    format!("Pomowise - {} {:02}:{:02}", snapshot.session_name, mins, secs)
}

fn find_pomowise_binary() -> String {
    // Check if pomowise is in PATH
    let cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
    if let Ok(output) = Command::new(cmd).arg("pomowise").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return path;
            }
        }
    }

    // Fallback to ~/.pomowise/bin/
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_default();
    let bin = if cfg!(target_os = "windows") { "pomowise.exe" } else { "pomowise" };
    let path = std::path::PathBuf::from(&home).join(".pomowise").join("bin").join(bin);
    path.to_string_lossy().to_string()
}

fn open_tui() {
    let binary = find_pomowise_binary();

    #[cfg(target_os = "linux")]
    {
        let terminals = ["kitty", "alacritty", "foot", "wezterm", "gnome-terminal", "konsole", "xterm"];
        for term in &terminals {
            if Command::new("which").arg(term).output().map(|o| o.status.success()).unwrap_or(false) {
                let _ = match *term {
                    "gnome-terminal" => Command::new(term).arg("--").arg(&binary).spawn(),
                    "konsole" => Command::new(term).arg("-e").arg(&binary).spawn(),
                    _ => Command::new(term).arg("-e").arg(&binary).spawn(),
                };
                return;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Use osascript to open Terminal.app and run the binary
        let script = format!(
            "tell application \"Terminal\" to do script \"{}\"",
            binary.replace('\"', "\\\"")
        );
        let _ = Command::new("osascript").arg("-e").arg(&script).spawn();
    }

    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("cmd").args(["/c", "start", "cmd", "/c", &binary]).spawn();
    }
}

fn main() {
    init_event_loop();

    let menu = Menu::new();
    let open_item = MenuItem::new("Open Pomowise", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = menu.append(&open_item);
    let _ = menu.append(&quit_item);

    let initial_icon = create_icon_with_time((128, 128, 128), 0, 0);

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Pomowise - Idle")
        .with_icon(initial_icon)
        .build()
        .expect("Failed to create tray icon");

    let open_id = open_item.id().clone();
    let quit_id = quit_item.id().clone();

    // Main loop - poll IPC + handle menu events (single-threaded)
    loop {
        process_events();

        // Check menu events
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == open_id {
                open_tui();
            } else if event.id == quit_id {
                break;
            }
        }

        // Update tray from IPC status file
        if let Ok(snapshot) = ipc::read_status() {
            let tooltip = format_tooltip(&snapshot);
            let _ = tray.set_tooltip(Some(&tooltip));

            let mins = snapshot.remaining_secs / 60;
            let secs = snapshot.remaining_secs % 60;
            let bg = state_color(&snapshot);
            let icon = create_icon_with_time(bg, mins, secs);
            let _ = tray.set_icon(Some(icon));
        }

        thread::sleep(Duration::from_millis(100));
    }
}
