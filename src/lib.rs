use std::fmt::Write;
use std::os::windows::process::CommandExt;
use std::process::Command;

pub struct WinToastNotify {
    pub app_id: Option<String>,
    pub duration: Duration,
    pub notif_open: Option<String>,
    pub title: Option<String>,
    pub messages: Option<Vec<String>>,
    pub logo: Option<String>,
    pub logo_circle: CropCircle,
    pub image: Option<String>,
    pub image_placement: ImagePlacement,
    pub actions: Option<Vec<Action>>,
    pub audio: Option<Audio>,
    pub audio_loop: Loop,
}

impl Default for WinToastNotify {
    fn default() -> Self {
        Self::new()
    }
}

impl WinToastNotify {
    pub fn new() -> Self {
        Self {
            app_id: None,
            notif_open: None,
            duration: Duration::Short,
            title: None,
            messages: Some(vec![String::from("Hellow World")]),
            logo: None,
            logo_circle: CropCircle::False,
            image: None,
            image_placement: ImagePlacement::Top,
            actions: None,
            audio: Some(Audio::WinDefault),
            audio_loop: Loop::False,
        }
    }

    /// Set App ID
    /// 
    /// The default App ID is PowerShell.
    /// 
    /// Run `Get-StartApps` in PowerShell to see a list of apps available to you.
    /// 
    /// # Examples:
    /// 
    /// APP Name: PowerShell - APP ID : `{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe`;
    /// 
    /// APP Name: WindowsTerminal - APP ID: `Microsoft.WindowsTerminal_8wekyb3d8bbwe!App`;
    /// 
    /// APP Name: Microsoft Edge - APP ID: `MSEdge`.
    /// 
    /// # Warning:
    /// 
    /// After you set an APP ID that does not exist in the system, please set `set_notif_open("")` to ensure that notifications can be delivered, and the notification without app icon
    /// ```
    /// use win_toast_notify::*; 
    /// 
    /// WinToastNotify::new()
    ///     .set_app_id("app name")
    ///     .set_notif_open("")
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    /// 
    /// # Note
    ///
    /// If you want to set an app id that does not exist on the system and display the app icon, you need to add the following entry to the registry in administrator mode
    /// 
    /// Create a new key under `HKEY_CLASSES_ROOT\AppUserModelId` with the name of your APP ID.
    /// 
    /// Create the following new strings under this key:
    /// 
    /// - Name: `DisplayName`, Value: Your APP name
    /// 
    /// - Name: `IconUrl`, Value: Path to your APP icon
    pub fn set_app_id(mut self, id: &str) -> Self {
        self.app_id = Some(id.into());
        self
    }

    /// Duration of notification.
    pub fn set_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Open link when notification is clicked.
    /// # Examples
    /// ```
    /// use win_toast_notify::*; 
    /// 
    /// WinToastNotify::new()
    ///     .set_notif_open("https://www.google.com/")    // "C:/Windows/Web/Screen/img104.jpg"
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    pub fn set_notif_open(mut self, url_or_path: &str) -> Self {
        self.notif_open = Some(url_or_path.trim().into());
        self
    }

    /// Set the title of the notification.
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the messages of the notification.
    /// 
    /// # Note
    /// Only supports adding two messages, but line breaks are allowed in the message content.
    /// # Examples
    /// ```
    /// use win_toast_notify::*; 
    /// 
    /// WinToastNotify::new()
    ///     .set_messages(vec![
    ///         "Heads up The wheels are spinning\nAcross the plains in valleys deep",
    ///         "To dawn the wheels that sing\nAn unending dream"
    ///     ])
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    pub fn set_messages(mut self, messages: Vec<&str>) -> Self {
        self.messages = Some(Box::new(messages.iter().map(|t| t.to_string())).collect());
        self
    }

    /// Set the notification logo and specify whether to crop it into a circle.
    pub fn set_logo(mut self, path: &str, hint_crop: CropCircle) -> Self {
        self.logo = Some(path.trim().into());
        self.logo_circle = hint_crop;
        self
    }

    /// Set the notification image and its position.
    pub fn set_image(mut self, path: &str, position: ImagePlacement) -> Self {
        self.image = Some(path.trim().into());
        self.image_placement = position;
        self
    }

    /// Set the actions.
    /// # Example
    /// ```
    /// // Add two buttons to the notification.
    /// use win_toast_notify::*;
    /// 
    /// WinToastNotify::new()
    ///     .set_actions(vec![
    ///         Action {
    ///             activation_type: ActivationType::Protocol,
    ///             action_content: "Open Url",
    ///             arguments: "https://www.google.com/",
    ///             image_url: None
    ///         },
    ///         Action {
    ///             activation_type: ActivationType::Protocol,
    ///             action_content: "Open File",
    ///             arguments: r"C:\Windows\Web\Screen\img104.jpg",
    ///             image_url: None
    ///         }
    ///     ])
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    /// # Note
    /// You can only have up to 5 buttons
    /// 
    /// [Microsoft Docs about Button](https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/adaptive-interactive-toasts?tabs=appsdk#buttons)
    pub fn set_actions(mut self, actions: Vec<Action>) -> Self {
        self.actions = Some(actions);
        self
    }

    /// Set the notification sound and whether the sound should loop.
    /// 
    /// Default is [Audio::WinDefault](enum.Audio.html)
    /// 
    /// # Example
    /// ```
    /// use win_toast_notify::*;
    /// 
    /// // Use system audio and loop it.
    /// WinToastNotify::new()
    ///     .set_audio(Audio::WinLoopingAlarm5, Loop::True)
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// 
    /// // Use other audio, but don't loop it.
    /// WinToastNotify::new()
    ///     .set_audio(Audio::From(r"C:\Windows\Media\Ring05.wav"), Loop::False)
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    pub fn set_audio(mut self, audio: Audio, audio_loop: Loop) -> Self {
        self.audio = Some(audio);
        self.audio_loop = audio_loop;
        self
    }

    /// Show the notification.
    pub fn show(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Create a String instance and preallocate 2000 bytes of memory for it, reduce the number of memory reallocations
        let mut command = String::with_capacity(2000);
        // Start of XML
        command.push_str("$xml = @\"");
        write!(
            command,
            r#"
            <toast{}{}>
                <visual>
                    <binding template="ToastGeneric">
                        {}
                        {}
                        {}
                        {}
                    </binding>
                </visual>
                <actions>
                    {}
                </actions>
                {}
            </toast>
            "#,
            match &self.notif_open {
                Some(url) => format!(r#" activationType="protocol" launch="{}""#, url),
                None => String::new(),
            },
            match &self.duration {
                Duration::Short => "",
                Duration::Long => " duration=\"long\"",
                Duration::TimeOut => " scenario=\"incomingCall\"",
            },
            match (&self.logo, &self.logo_circle) {
                (Some(logo), CropCircle::True) => format!(
                    "\n<image placement=\"appLogoOverride\" hint-crop=\"circle\" src=\"{}\"/>",
                    &logo
                ),
                (Some(logo), CropCircle::False) =>
                    format!("\n<image placement=\"appLogoOverride\" src=\"{}\"/>", &logo),
                (None, _) => String::new(),
            },
            match &self.title {
                Some(title) => format!("\n<text>{}</text>", &title),
                None => String::new(),
            },
            match &self.messages {
                Some(messages) => messages.iter().fold(String::new(), |acc, message| {
                    format!("{}\n<text>{}</text>", acc, message)
                }),
                None => String::new(),
            },
            match (&self.image, &self.image_placement) {
                (Some(image), ImagePlacement::Top) =>
                    format!("\n<image placement=\"hero\" src=\"{}\"/>", &image),
                (Some(image), ImagePlacement::Bottom) => format!("\n<image src=\"{}\"/>", &image),
                (None, _) => String::new(),
            },
            match &self.actions {
                Some(actions) => actions.iter().fold(String::new(), |acc, action| {
                    format!(
                        "{}\n<action content=\"{}\" activationType=\"{}\" arguments=\"{}\" {} />",
                        acc,
                        action.action_content,
                        action.activation_type.as_str(),
                        action.arguments,
                        action.image_url.map_or(String::new(), |url| format!("imageUri=\"{}\"", url)),
                    )
                }),
                None => String::new(),
            },
            match &self.audio {
                Some(audio) => match (audio, &self.audio_loop) {
                    (Audio::From(_), _) => String::from("\n<audio silent=\"true\" />"),
                    (Audio::Silent, _) => String::from("\n<audio silent=\"true\" />"),
                    (_, Loop::False) => format!("\n<audio src=\"{}\" />", audio.as_str()),
                    (_, Loop::True) =>
                        format!("\n<audio src=\"{}\" loop=\"true\" />", audio.as_str()),
                },
                None => String::from("\n<audio silent=\"true\" />"),
            }
        )?;
        // End of XML (The terminator ("@") cannot be preceded by a space)
        command.push_str("\n\"@");
        // Powershell commands that send Toast notifications
        write!(
            command,
            r#"
            $XmlDocument = [Windows.Data.Xml.Dom.XmlDocument, Windows.Data.Xml.Dom.XmlDocument, ContentType = WindowsRuntime]::New()
            $XmlDocument.loadXml($xml)
            $AppId = '{}'
            [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]::CreateToastNotifier($AppId).Show($XmlDocument)
            "#,
            match &self.app_id {
                Some(id) => id,
                None =>
                    r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe",
            }
        )?;
        // Add Audio Source
        if let Some(Audio::From(url)) = &self.audio {
            write!(
                command,
                r#"
                $MediaPlayer = [Windows.Media.Playback.MediaPlayer, Windows.Media, ContentType = WindowsRuntime]::New()
                $MediaPlayer.Source = [Windows.Media.Core.MediaSource]::CreateFromUri('{}')
                $MediaPlayer.Play()
                "#,
                url
            )?
        }
        // Run it by PowerShell
        let output = Command::new("powershell")
            .creation_flags(0x08000000)
            .args(["-Command", &command])
            .output()
            .map_err(|e| format!("Failed to execute process: {}", e))?;
        if !output.status.success() {
            return Err(format!(
                "Failed to execute command: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        Ok(())
    }
}

// Duration of notification
pub enum Duration {
    Short,
    Long,
    TimeOut,
}

pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: &'static str,
    pub arguments: &'static str,
    pub image_url: Option<&'static str>,
}

pub enum ActivationType {
    Protocol, // 使用协议激活功能启动不同的应用程序
    System,
    Background, // 触发相应的后台任务，而不会中断用户
    Foreground, // 启动前台应用程序（默认值）
}

impl ActivationType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivationType::Foreground => "foreground",
            ActivationType::Background => "background",
            ActivationType::Protocol => "protocol",
            ActivationType::System => "system",
        }
    }
}

// Crop the LOGO into a circle
pub enum CropCircle {
    True,
    False,
}

pub enum ImagePlacement {
    Top,
    Bottom,
}

// System Audio
pub enum Audio {
    From(&'static str),
    Silent,
    WinDefault,
    WinIM,
    WinMail,
    WinRemainder,
    WinSMS,
    WinLoopingAlarm1,
    WinLoopingAlarm2,
    WinLoopingAlarm3,
    WinLoopingAlarm4,
    WinLoopingAlarm5,
    WinLoopingAlarm6,
    WinLoopingAlarm7,
    WinLoopingAlarm8,
    WinLoopingAlarm9,
    WinLoopingAlarm10,
    WinLoopingCall1,
    WinLoopingCall2,
    WinLoopingCall3,
    WinLoopingCall4,
    WinLoopingCall5,
    WinLoopingCall6,
    WinLoopingCall7,
    WinLoopingCall8,
    WinLoopingCall9,
    WinLoopingCall10,
}

impl Audio {
    pub fn as_str(&self) -> &'static str {
        match self {
            Audio::From(url) => url,
            Audio::Silent => "",
            Audio::WinDefault => "ms-winsoundevent:Notification.Default",
            Audio::WinIM => "ms-winsoundevent:Notification.IM",
            Audio::WinMail => "ms-winsoundevent:Notification.Mail",
            Audio::WinRemainder => "ms-winsoundevent:Notification.Remainder",
            Audio::WinSMS => "ms-winsoundevent:Notification.SMS",
            Audio::WinLoopingAlarm1 => "ms-winsoundevent:Notification.Looping.Alarm",
            Audio::WinLoopingAlarm2 => "ms-winsoundevent:Notification.Looping.Alarm2",
            Audio::WinLoopingAlarm3 => "ms-winsoundevent:Notification.Looping.Alarm3",
            Audio::WinLoopingAlarm4 => "ms-winsoundevent:Notification.Looping.Alarm4",
            Audio::WinLoopingAlarm5 => "ms-winsoundevent:Notification.Looping.Alarm5",
            Audio::WinLoopingAlarm6 => "ms-winsoundevent:Notification.Looping.Alarm6",
            Audio::WinLoopingAlarm7 => "ms-winsoundevent:Notification.Looping.Alarm7",
            Audio::WinLoopingAlarm8 => "ms-winsoundevent:Notification.Looping.Alarm8",
            Audio::WinLoopingAlarm9 => "ms-winsoundevent:Notification.Looping.Alarm9",
            Audio::WinLoopingAlarm10 => "ms-winsoundevent:Notification.Looping.Alarm10",
            Audio::WinLoopingCall1 => "ms-winsoundevent:Notification.Looping.Call",
            Audio::WinLoopingCall2 => "ms-winsoundevent:Notification.Looping.Call2",
            Audio::WinLoopingCall3 => "ms-winsoundevent:Notification.Looping.Call3",
            Audio::WinLoopingCall4 => "ms-winsoundevent:Notification.Looping.Call4",
            Audio::WinLoopingCall5 => "ms-winsoundevent:Notification.Looping.Call5",
            Audio::WinLoopingCall6 => "ms-winsoundevent:Notification.Looping.Call6",
            Audio::WinLoopingCall7 => "ms-winsoundevent:Notification.Looping.Call7",
            Audio::WinLoopingCall8 => "ms-winsoundevent:Notification.Looping.Call8",
            Audio::WinLoopingCall9 => "ms-winsoundevent:Notification.Looping.Call9",
            Audio::WinLoopingCall10 => "ms-winsoundevent:Notification.Looping.Call10",
        }
    }
}

// 音频循环
pub enum Loop {
    True,
    False,
}