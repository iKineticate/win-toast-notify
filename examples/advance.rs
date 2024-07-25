use win_toast_notify::*;
use std::env;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let logo_path = current_dir.clone().join("examples/album_artist.png");
    let image_path = current_dir.join("examples/album_cover.jpg");
    let introduce_url = "https://honkai-star-rail.fandom.com/wiki/Hope_Is_the_Thing_With_Feathers";
    let music_url = "https://t.co/6urFxrI6K0";
    let music_lyric = "https://x.com/honkaistarrail/status/1789149010831569254";

    WinToastNotify::new()
        .set_open(introduce_url)    // 点击通知的打开链接或文件(夹)
        .set_duration(Duration::Long)
        .set_title("Hope Is the Thing With Feathers - Robin")
        .set_messages(vec![
            "Heads up the wheels are spinning\nAcross the plains in valleys deep",
            "To dawn the wheels that sing\nAn unending dream"
        ])
        .set_logo(logo_path.to_str().expect("Failed to convert path to string"), CropCircle::True)
        .set_image(image_path.to_str().expect("Failed to convert path to string"), ImagePlacement::Top)
        .set_actions(vec![
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Listen",
                arguments: music_url.to_string(),
                image_url: None,
            },
            Action {
                activation_type: ActivationType::Protocol,
                action_content: "Lyric",
                arguments: music_lyric.to_string(),
                image_url: None,
            }
        ])
        .set_audio(Audio::WinLoopingAlarm5, Loop::True)
        .show()
        .expect("Failed to show toast notification")
}