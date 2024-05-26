# win-toast-notiy
这个仓库是我个人使用的，用于尝试在 Rust 中发送 Windows Toast 通知。作为一个 Rust 初学者，我的代码可能不是最佳实践，可能会有一些不完美之处。我主要参考了以下两个项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

## 目的

目前Windows Toast通知库在ARM64上编译错误、PowerShell腳本默认禁止运行或其他不足，因此创建此仓库，并辅以学习如何在 Rust 中使用 PowerShell 发送 Windows Toast 通知。虽然这不是一个完美的解决方案，但我希望通过这个项目能够学到一些东西。

## 使用
```
#Cargo.toml
[dependencies]
win-toast-notify = "0.1.0"
```

## 例子
```rust
use win_toast_notify::*;

fn main() {
    WinToastNotify::new()
        // .set_app_id("App Name or App ID")    // 默认PowerShell，若输入系统不存在的AppID，则需使用fn set_notif_open("")，才可发送通知
        .set_notif_open("https://en.wikipedia.org/wiki/Li_Qingzhao")    // 点击通知的打开链接或文件(夹)
        .set_duration(Duration::Long)
        .set_title("《一剪梅·红藕香残玉簟秋》 宋·李清照")
        .set_messages(vec![
            "红藕香残玉簟秋。轻解罗裳，独上兰舟。\n云中谁寄锦书来，雁字回时，月满西楼。",
            "花自飘零水自流。一种相思，两处闲愁。\n此情无计可消除，才下眉头，却上心头。"
        ])
        .set_logo(r"C:\Users\11593\Downloads\Li Qingzhao.jpeg", CropCircle::True)
        .set_image(r"C:\Users\11593\Downloads\yijianmei.jpg", ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "阅读",
                arguments: "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#1",
                image_url: Some(r"C:\Users\11593\Documents\GitHub\win-toast-notify\examples\basic\src\read.png")
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "赏析",
                arguments: "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#4",
                image_url: Some(r"C:\Users\11593\Documents\GitHub\win-toast-notify\examples\basic\src\appreciation.png")
            }
        ])
        .set_audio(Audio::WinLoopingAlarm5, Loop::True)
        .show()
        .expect("Failed to show toast notification")
}
```

![image](https://github.com/iKineticate/win-toast-notify/examples/images/example_zh.png)

## 问题

1.若设置系统不存在的AppID，则需设置`set_notif_open("")`，否则无法发送通知

2.终端输入"Get-StartApps"可查询系统已存在的AppID

3.不明原因导致无法播放除系统音频以外的音频资源
