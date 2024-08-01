use win_toast_notify::{WinToastNotify, Action, ActivationType};

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let button_read_path = current_dir.join("examples/button_read.png");
    let button_appreciation_path = current_dir.join("examples/button_appreciation.png");

    WinToastNotify::new()
        .set_title("Rust")
        .set_messages(vec![
            "A language empowering everyone",
            "to build reliable and efficient software."
        ])
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: String::new(),
                arguments: "https://doc.rust-lang.org/book/".to_string(),
                image_url: Some(button_appreciation_path.to_string_lossy().to_string()),
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: String::new(),
                arguments: r"C:\Windows\Web\Screen\img101.jpg".to_string(),
                image_url: Some(button_read_path.to_string_lossy().to_string()),
            }
        ])
        .show()
        .expect("Failed to show toast notification")
}