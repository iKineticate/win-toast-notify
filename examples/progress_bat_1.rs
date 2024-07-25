use win_toast_notify::{WinToastNotify, Duration, Progress};

fn main() {
    let tag = "weekly-playlist";
    let title = "Weekly playlist";
    let mut status = String::from("Downloading...");
    let mut value = 0.0;
    let mut value_string = String::from("0/10 songs");

    WinToastNotify::new()
        .set_duration(Duration::Long)
        .set_title("Downloading your weekly playlist...")
        .set_progress(Progress {tag, title, status, value, value_string})
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        value = i as f32 / 10.0;
        if i != 10 {
            value_string = format!("{}/10 songs", i);
            WinToastNotify::progress_update(None, tag, value, value_string).expect("Failed to update");
        } else {
            status = String::from("Completed");
            value_string = String::from("10/10 songs");
            WinToastNotify::progress_complete(None, tag, status, value_string).expect("Failed to complete");
        };
    };
}