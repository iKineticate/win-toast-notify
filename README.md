<h3 align="center"> English | <a href='./README_zh.md'>简体中文</a></h3>

# win-toast-notiy
This is a library for sending Windows Toast notifications, mainly referencing the following two projects:

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

Tested in Windows 11

[0.1.3 Documentaton](https://docs.rs/win-toast-notify)

## Usage
```
#Cargo.toml
[dependencies]
win-toast-notify = "0.1.3"
```

## Examples
```rust
use win_toast_notify::*;

fn main() {
    WinToastNotify::new()
        // .set_app_id("App Name or App ID")
        .set_notif_open("https://honkai-star-rail.fandom.com/wiki/Hope_Is_the_Thing_With_Feathers")
        .set_duration(Duration::Long)
        .set_title("Hope Is the Thing With Feathers - Robin")
        .set_messages(vec![
            "Heads up The wheels are spinning\nAcross the plains in valleys deep",
            "To dawn the wheels that sing\nAn unending dream"
        ])
        .set_logo(r"C:\Users\11593\Documents\GitHub\win-toast-notify\examples\images\logo.png", CropCircle::True)
        .set_image(r"C:\Users\11593\Documents\GitHub\win-toast-notify\examples\images\Robin.jpg", ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Listen",
                arguments: "https://t.co/6urFxrI6K0",
                image_url: None
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Lyric",
                arguments: "https://x.com/honkaistarrail/status/1789149010831569254",
                image_url: None
            }
        ])
        .set_audio(Audio::WinLoopingAlarm5, Loop::True)
        .show()
        .expect("Failed to show toast notification")
}
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/examples/images/example_en.png)

## Issues

1.After you set an APP ID that does not exist in the system, please set `set_notif_open("")` to ensure that notifications can be delivered, and the notification without app icon

2.Currently unable to play other audio sources for unknown reasons
