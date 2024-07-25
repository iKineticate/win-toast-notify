use win_toast_notify::{WinToastNotify, Action, ActivationType};

fn main() {
    WinToastNotify::new()
        .set_title("Hellow World")
        .set_messages(vec![
            "There are two buttons here"
        ])
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Open Github",
                arguments: "https://github.com/",
                image_url: None,
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Open Wallpaper",
                arguments: r"C:\Windows\Web\Screen\img101.jpg",
                image_url: None,
            }
        ])
        .show()
        .expect("Failed to show toast notification")
}