# win-toast-notif-rs
这个仓库是我个人使用的，用于尝试在 Rust 中发送 Windows 吐司通知。作为一个 Rust 初学者，我的代码可能不是最佳实践，可能会有一些不完美之处。我主要参考了以下两个项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

## 目的

我创建这个仓库的目的是学习如何在 Rust 中使用 PowerShell 发送 Windows 吐司通知。虽然这不是一个完美的解决方案，但我希望通过这个项目能够学到一些东西。

## 使用说明

这个仓库中的代码可能并不适合用于生产环境，仅供学习和参考。如果你想要在自己的 Rust 项目中添加吐司通知功能，建议你查阅更为成熟和稳定的库，并仔细阅读相关文档。

将win_toast_notif.rs放至src目录中使用
```rust
use win_toast_notif::*;
mod win_toast_notif;

fn main() {
    WinToastNotif::new()
      .set_logo(r"C:\Windows\IdentityCRL\WLive48x48.png", LogoCropCircle::True)
      .set_title("Here's the title.")
      .set_messages(vec!["Hellow", "World"])
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
      .show()
}
```
![notif_screen](https://github.com/iKineticate/win-toast-notif-rs/assets/115683118/4a35bb13-5f18-4422-8bad-729d15756ecb)

## 免责声明

请注意，这个仓库中的代码仅供参考和学习之用，可能存在 bug 和不足之处。对于任何因使用该代码而造成的损失，我概不负责。
