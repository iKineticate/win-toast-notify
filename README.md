# win-toast-notif-rs
这个仓库是我个人使用的，用于尝试在 Rust 中发送 Windows 吐司通知。作为一个 Rust 初学者，我的代码可能不是最佳实践，可能会有一些不完美之处。我主要参考了以下两个项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

## 目的

我创建这个仓库的目的是学习如何在 Rust 中使用 PowerShell 发送 Windows 吐司通知。虽然这不是一个完美的解决方案，但我希望通过这个项目能够学到一些东西。

## 使用说明

这个仓库中的代码可能并不适合用于生产环境，仅供学习和参考。如果你想要在自己的 Rust 项目中添加吐司通知功能，建议你查阅更为成熟和稳定的库，并仔细阅读相关文档。

将win_toast_notif.rs添加至src目录中
```rust
use win_toast_notif::*;
mod win_toast_notif;

fn main() {
    WinToastNotif::new()
        .set_notif_open("https://www.google.com/")
        .set_duration(Duration::Long)
        .set_title("Here's the title.")
        .set_messages(vec!["Hellow", "World"])
        .set_logo(r"C:\Windows\IdentityCRL\WLive48x48.png", LogoCropCircle::True)
        .set_image(r"C:\Windows\Web\Screen\img100.jpg", ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: String::from("Open File"),
                arguments: String::from(r"C:\Windows\Web\Screen\img100.jpg"),
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: String::from("Open Url"),
                arguments: String::from("https://www.google.com/"),
            }
        ])
        // .set_audio(Audio::LoopingAlarm1, Loop::True)        
        // .set_audio_source("https://nyanpass.com/nyanpass.mp3")
        .show()
}
```
![yijianmei](https://github.com/iKineticate/win-toast-notif-rs/assets/115683118/2d0c7908-3fd1-445a-8f27-7fbb4feefd0e)


## 免责声明

请注意，这个仓库中的代码仅供参考和学习之用，可能存在 bug 和不足之处。对于任何因使用该代码而造成的损失，我概不负责。
