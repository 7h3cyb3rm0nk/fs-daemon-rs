use notify::{RecommendedWatcher, Watcher, RecursiveMode, event::Event};
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

use notify_rust::{Notification, Urgency};

mod file_watcher;
mod notifier;

fn main() {
    if let Err(err) = run_daemon() {
        eprintln!("Error: {}", err);
    }
}



fn run_daemon() -> Result<(), Box<dyn std::error::Error>> {
    let mut watcher = file_watcher::FileWatcher::new()?;
    watcher.watch_directory("/home");
    watcher.watch_directory("/opt/");


    loop {
        match watcher.receive_event() {
            Ok(event) => handle_event(event),
            Err(err) => eprintln!("Watcher error: {}", err),
        }
    }
}

type DebouncedEvent = Event;
fn handle_event(event: Event) {
    match event {
        DebouncedEvent::kind::Create(path) => {
            notifier::send_notification("File Created", &path.to_string_lossy());   
        }

        DebouncedEvent::kind::Access(path) => {
            notifier::send_notification("File Written", &path.to_string_lossy());
        }
        
        DebouncedEvent::Remove(path) => {
            notifier::send_notification("File Deleted", &path.to_string_lossy());
        }

        DebouncedEvent::Rename(old_path, new_path) => {
            notifier::send_notification("File Moved", &format!("{} to {}", old_path.to_string_lossy(), new_path.to_string_lossy()));
        }

        DebouncedEvent::NoticeAccess(path) => {
            notifier::send_notification("File Accessed", &path.to_string_lossy());
        }

        DebouncedEvent::NoticeModify(path) => {
            notifier::send_notification("File Modified", &path.to_string_lossy());
        }
        _ => {}

    }
}
