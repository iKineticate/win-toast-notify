use win_toast_notify::{WinToastNotify, Duration, Progress};

fn main() {
    WinToastNotify::new()
        .set_duration(Duration::Long)
        .set_title("Downloading your weekly playlist...")
        .set_progress(Progress {
            tag: "weekly-playlist",
            title:"Weekly playlist",
            status:"Downloading...",
            value: 0.0, 
            value_string: "0/10 songs"
        })
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let i_f32 = i as f32 / 10.0;
        if i != 10 {
            WinToastNotify::progress_update(None, "weekly-playlist", i_f32, &format!("{}/10 songs", i)).expect("Failed to update");
        } else {
            WinToastNotify::progress_complete(None, "weekly-playlist", "Completed", "10/10 songs").expect("Failed to complete");
        };
    };
}