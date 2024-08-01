use win_toast_notify::{WinToastNotify, Action, ActivationType};

fn main() {
    WinToastNotify::new()
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Url".to_string(),
                arguments: "https://www.google.com/".to_string(),
                image_url: None
            },
            Action {
                activation_type: ActivationType::Protocol,
                 action_content: "File".to_string(),
                 arguments: r"C:\Windows\Web\Screen\img104.jpg".to_string(),
                 image_url: None
             },
            Action {
                 activation_type: ActivationType::Protocol,
                 action_content: "Folder".to_string(),
                 arguments: r"$env:USERPROFILE\Downloads".to_string(),   // PowerShell supports using environment variables
                 image_url: None
             }
         ])
         .show()
         .expect("Failed to show toast notification");
}