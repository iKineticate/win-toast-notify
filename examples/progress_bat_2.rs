use win_toast_notify::{WinToastNotify, CropCircle, Duration, Progress};
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.join("examples/progress_logo.png");

    WinToastNotify::new()
        .set_duration(Duration::Long)   
        .set_title("Downloading miHoYo Game...")
        .set_messages(vec![
            "May This Journey Lead Us Starward"
        ])
        .set_logo(logo_path.to_str().expect("Failed to convert path to string"), CropCircle::True)
        .set_progress(Progress {
            tag: "star-rail",
            title:"Honkai: Star Rail",
            status:"Downloading...",
            value: 0.0,
            value_string: "0%"
        })
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let i_f32 = i as f32 / 10.0;
        if i != 10 {
            WinToastNotify::progress_update(None, "star-rail", i_f32, &format!("{:.1}%", i_f32 * 100.0)).expect("Failed to update");
        } else {
            WinToastNotify::progress_complete(None, "star-rail", "Completed", "100%").expect("Failed to complete");
        };
    };
}