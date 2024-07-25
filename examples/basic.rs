use win_toast_notify::WinToastNotify;

fn main() {
    WinToastNotify::new()
        .set_title("Title")
        .set_messages(vec![
            "This is a simple toast message"
        ])
        .show()
        .expect("Failed to show toast notification")
}