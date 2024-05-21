# win-toast-notif-rs
这个仓库是我个人使用的，用于尝试在 Rust 中发送 Windows Toast 通知。作为一个 Rust 初学者，我的代码可能不是最佳实践，可能会有一些不完美之处。我主要参考了以下两个项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

## 目的

我创建这个仓库的目的是学习如何在 Rust 中使用 PowerShell 发送 Windows Toast 通知。虽然这不是一个完美的解决方案，但我希望通过这个项目能够学到一些东西。

## 使用说明

这个仓库中的代码可能并不适合用于生产环境，仅供学习和参考。如果你想要在自己的 Rust 项目中添加 Toast 通知功能，建议你查阅更为成熟和稳定的库，并仔细阅读相关文档。

将win_toast_notif.rs添加至src目录中
```rust
use win_toast_notif::*;
mod win_toast_notif;

fn main() {
    WinToastNotif::new()
        .set_app_id(" Your App Name ")    // 默认PowerShell，若需其他App Id，终端输入"Get-StartApps"获取
        .set_notif_open("https://en.wikipedia.org/wiki/Li_Qingzhao")    // 点击通知的打开链接或文件(夹)
        .set_duration(Duration::Long)
        .set_title("《一剪梅·红藕香残玉簟秋》 宋·李清照")
        .set_messages(vec![
            "红藕香残玉簟秋。轻解罗裳，独上兰舟。\n云中谁寄锦书来，雁字回时，月满西楼。",
            "花自飘零水自流。一种相思，两处闲愁。\n此情无计可消除，才下眉头，却上心头。"])
        .set_logo(r"C:\Users\11593\Downloads\Li Qingzhao.jpeg", CropCircle::True)
        .set_image(r"C:\Users\11593\Downloads\yijianmei.jpg", ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "阅读",
                arguments: "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#1",
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "赏析",
                arguments: "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#4",
            }
        ])
        // .set_audio(Audio::WinLoopingAlarm1, Loop::True)
        // .set_audio(Audio::From(r"C:\Windows\Media\Ring05.wav"), Loop::True) 
        .show()
}
```

![yijianmei_screen](https://github.com/iKineticate/win-toast-notif-rs/assets/115683118/64c01312-9507-4423-8e43-cd3be37d8e8d)

## 免责声明

请注意，这个仓库中的代码仅供参考和学习之用，可能存在 bug 和不足之处。对于任何因使用该代码而造成的损失，本人概不负责。
