use notify_rust::Notification;

pub fn notify_session_end(session_type: &str) {
    Notification::new()
        .summary("Pomodoro")
        .body(&format!("{} complete!", session_type))
        .show()
        .ok();
}
