# win-toast-notiy

这是一个发送Windows Toast通知的库，主要参考了以下两个项目：

- [wpush.rs](https://github.com/saez-juan/wpush.rs)
- [toast-notification-examples](https://github.com/GitHub30/toast-notification-examples)



[查看文档](https://docs.rs/win-toast-notify)

## 使用
```toml
#Cargo.toml
[dependencies]
win-toast-notify = "0.1.4"
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
                arguments: "https://github.com/".to_string(),
                image_url: None,
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Open Wallpaper",
                arguments: r"C:\Windows\Web\Screen\img101.jpg".to_string(),
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
    let logo_path = current_dir.clone().join("examples/poet.jpeg");
    let image_path = current_dir.clone().join("examples/poetry.jpg");
    let button_read_path = current_dir.clone().join("examples/button_read.png");
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
        .set_logo(logo_path.to_str().expect("Failed to convert path to string"), CropCircle::True)
        .set_image(image_path.to_str().expect("Failed to convert path to string"), ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "阅读",
                arguments: read_url.to_string(),
                image_url: Some(button_read_path.to_string_lossy().into_owned()),
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "赏析",
                arguments: appreciation_url.to_string(),
                image_url: Some(button_appreciation_path.to_string_lossy().into_owned()),
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

    WinToastNotify::new()
        .set_duration(Duration::Long)   
        .set_title("Downloading miHoYo Game...")
        .set_messages(vec![
            "May This Journey Lead Us Starward"
        ])
        .set_logo(logo_path.to_str().expect("Failed to convert path to string"), CropCircle::True)
        .set_progress(Progress {
            tag: "star-rail",
            title:"Honkai: Star Rail",
            status:"Downloading...",
            value: 0.0,
            value_string: "0%"
        })
        .show()
        .expect("Failed to show toast notification");

    for i in 1..=10 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let i_f32 = i as f32 / 10.0;
        if i != 10 {
            WinToastNotify::progress_update(None, "star-rail", i_f32, &format!("{:.1}%", i_f32 * 100.0)).expect("Failed to update");
        } else {
            WinToastNotify::progress_complete(None, "star-rail", "Completed", "100%").expect("Failed to complete");
        };
    };
}
```

![image](https://raw.githubusercontent.com/iKineticate/win-toast-notify/main/screenshots/progress.gif)