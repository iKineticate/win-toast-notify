use win_toast_notif::*;
mod win_toast_notif;

fn main() {
    WinToastNotif::new()
        .set_notif_open("https://www.google.com/")
        .set_duration(Duration::Long)
        .set_title("Here's the title.")
        .set_messages(vec!["Hellow", "World"])
        .set_logo(r"C:\Windows\IdentityCRL\WLive48x48.png", LogoCropCircle::True)
        .set_image(r"C:\Windows\Web\Screen\img102.jpg", ImagePlacement::Top)
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
        .set_audio(Audio::LoopingAlarm1, Loop::True)    // .set_audio_source(r"C:\Program Files\Microsoft Office\root\Office16\sdxs\FA000000084\fluidhost\static\media\wl_completion_sound.13a69e47a3545eb5ac81.mp3")
        .show()
}
