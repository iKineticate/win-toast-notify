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

