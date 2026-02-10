use std::io;
use std::path::PathBuf;

use crate::timer::TimerSnapshot;

/// Path to the IPC status file
pub fn status_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    home.join(".pomowise").join("status.json")
}

/// Write a snapshot to the status file as JSON
pub fn write_status(snapshot: &TimerSnapshot) -> io::Result<()> {
    let path = status_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string(snapshot)?;
    std::fs::write(&path, json)?;
    Ok(())
}

/// Read the current status from the status file
pub fn read_status() -> io::Result<TimerSnapshot> {
    let path = status_path();
    let json = std::fs::read_to_string(&path)?;
    let snapshot: TimerSnapshot = serde_json::from_str(&json)?;
    Ok(snapshot)
}

/// Remove the status file on exit
pub fn cleanup() {
    let path = status_path();
    let _ = std::fs::remove_file(&path);
}
