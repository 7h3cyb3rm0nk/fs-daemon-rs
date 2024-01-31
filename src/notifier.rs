use notify_rust::{Notification, Urgency};

pub fn send_notification(title: &str, message: &str) {
    Notification::new()
        .summary(title)
        .body(message)
        .urgency(Urgency::Normal)
        .show()
        .expect("Failed to send notification");
}
