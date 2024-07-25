use win_toast_notify::{WinToastNotify, CropCircle, Duration, Progress};
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.join("examples/progress_logo.png");

    let tag = "star-rail";
    let title = "Honkai: Star Rail";
    let mut status = String::from("Downloading...");
    let mut value = 0.0;
    let mut value_string = String::from("0%");

    WinToastNotify::new()
        .set_duration(Duration::Long)   
        .set_title("Downloading miHoYo Game...")
        .set_messages(vec![
            "May This Journey Lead Us Starward"
        ])
        .set_logo(logo_path.to_str().expect("Failed to convert path to string"), CropCircle::True)
        .set_progress(Progress {tag, title, status, value, value_string} )
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        value = i as f32 / 10.0;
        if i != 10 {
            value_string = format!("{:.1}%", value * 100.0);
            WinToastNotify::progress_update(None, tag, value, value_string).expect("Failed to update");
        } else {
            status = String::from("Completed");
            value_string = String::from("100%");
            WinToastNotify::progress_complete(None, tag, status, value_string).expect("Failed to complete");
        };
    };
}