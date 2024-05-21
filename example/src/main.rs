// #![windows_subsystem = "windows"]
use win_toast_notif::*;
mod win_toast_notif;

fn main() {
    WinToastNotif::new()
        .set_app_id(" ")    // 默认是PowerShell，若需要其他名称，终端输入"Get-StartApps"，获取其他App Id
        .set_notif_open("https://en.wikipedia.org/wiki/Li_Qingzhao")
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
