# win-toast-notiy

**Win Toast Notify** 是一个用于发送 Windows Toast 通知的 Rust 库。该库主要参考了以下项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)

该库已经在 Windows 11 上进行了测试。

**重要通知：** 该库目前处于不稳定状态。

## 文档

有关详细的使用和 API 参考，请参阅[文档](https://docs.rs/win-toast-notify)。

## 更新日志

有关最近的更改和更新，请参阅[更新日志](./CHANGELOG.md)。


## 使用
```toml
#Cargo.toml
[dependencies]
win-toast-notify = "0.1.5"
```

## 例子

### [文本](./examples/basic.rs)
```PowerShell
cargo run --example basic
```
```rust
use win_toast_notify::WinToastNotify;

fn main() {
    WinToastNotify::new()
        .set_title("Title")
        .set_messages(vec![
            "This is a simple toast message"
        ])
        .show()
        .expect("Failed to show toast notification")
}
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/basic.png)

### [按钮](./examples/button.rs)
```PowerShell
cargo run --example button
```
```rust
use win_toast_notify::{WinToastNotify, Action, ActivationType};

fn main() {
    WinToastNotify::new()
        .set_title("Hellow World")
        .set_messages(vec!["There are two buttons here"])
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
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/button_basic.png)

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/button_image.png)

### [古诗](./examples/advance_2.rs)
```PowerShell
cargo run --example advance_2
```
```rust
use win_toast_notify::*;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.join("examples/poet.jpeg");
    let image_path = current_dir.join("examples/poetry.jpg");
    let button_read_path = current_dir.join("examples/button_read.png");
    let button_appreciation_path = current_dir.join("examples/button_appreciation.png");
    let introduce_url = "https://en.wikipedia.org/wiki/Li_Qingzhao";
    let read_url = "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#1";
    let appreciation_url = "https://baike.baidu.com/item/%E4%B8%80%E5%89%AA%E6%A2%85%C2%B7%E7%BA%A2%E8%97%95%E9%A6%99%E6%AE%8B%E7%8E%89%E7%B0%9F%E7%A7%8B/593597#4";

    WinToastNotify::new()
        .set_open(introduce_url)    // 点击通知的打开链接或文件(夹)
        .set_duration(Duration::Long)
        .set_title("《一剪梅·红藕香残玉簟秋》 宋·李清照")
        .set_messages(vec![
            "红藕香残玉簟秋。轻解罗裳，独上兰舟。\n云中谁寄锦书来，雁字回时，月满西楼。",
            "花自飘零水自流。一种相思，两处闲愁。\n此情无计可消除，才下眉头，却上心头。"
        ])
        .set_logo(logo_path.to_str().expect("Path is an invalid unicode"), CropCircle::True)
        .set_image(image_path.to_str().expect("Path is an invalid unicode"), ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "阅读",
                arguments: read_url,
                image_url: Some(button_read_path.to_str().expect("Path is an invalid unicode")),
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "赏析",
                arguments: appreciation_url,
                image_url: Some(button_appreciation_path.to_str().expect("Path is an invalid unicode")),
            }
        ])
        .set_audio(Audio::WinLoopingAlarm5, Loop::True)
        .show()
        .expect("Failed to show toast notification")
}
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/advance_zh.png)

### [进度条](./examples/progress_bat_2.rs)
```PowerShell
cargo run --example progress_bat_2
```
```rust
use win_toast_notify::{WinToastNotify, CropCircle, Duration, Progress};
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.join("examples/progress_logo.png");

    let tag = "star-rail";
    let title = "Honkai: Star Rail";
    let mut status = String::from("Downloading...");
    let mut value = 0.0;
    let mut value_string = String::from("0%");

    WinToastNotify::new()
        .set_duration(Duration::Long)   
        .set_title("Downloading miHoYo Game...")
        .set_messages(vec![
            "May This Journey Lead Us Starward"
        ])
        .set_logo(logo_path.to_str().expect("Path is an invalid unicode"), CropCircle::True)
        .set_progress(Progress {tag, title, status, value, value_string} )
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        value = i as f32 / 10.0;
        if i != 10 {
            value_string = format!("{:.1}%", value * 100.0);
            WinToastNotify::progress_update(None, tag, value, value_string).expect("Failed to update");
        } else {
            status = String::from("Completed");
            value_string = String::from("100%");
            WinToastNotify::progress_complete(None, tag, status, value_string).expect("Failed to complete");
        };
    };
}
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/progress.gif)