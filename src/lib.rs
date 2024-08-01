use std::fmt::Write;
use std::os::windows::process::CommandExt;
use std::process::Command;
use xml::escape::escape_str_attribute;

pub struct WinToastNotify {
    pub app_id: Option<String>,
    pub duration: Duration,
    pub scenario: Scenario,
    pub open: Option<String>,
    pub title: Option<String>,
    pub messages: Option<Vec<String>>,
    pub logo: Option<String>,
    pub logo_circle: CropCircle,
    pub image: Option<String>,
    pub image_placement: ImagePlacement,
    pub actions: Option<Vec<Action>>,
    pub progress: Option<Progress>,
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
            open: None,
            duration: Duration::Short,
            scenario: Scenario::None,
            title: None,
            messages: None,
            logo: None,
            logo_circle: CropCircle::False,
            image: None,
            image_placement: ImagePlacement::Top,
            actions: None,
            audio: Some(Audio::WinDefault),
            audio_loop: Loop::False,
            progress: None,
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
    /// After you set an APP ID that does not exist in the system, please set `set_open("")` to ensure that notifications can be delivered, and the notification without app icon
    /// ```
    /// use win_toast_notify::WinToastNotify;
    /// 
    /// WinToastNotify::new()
    ///     .set_app_id(r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe")
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
        self.app_id = Some(escape_str_attribute(id).into_owned());
        self
    }

    /// Duration of notification.
    pub fn set_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// [Microsoft Docs about Scenario](https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-toast#:~:text=None-,scenario,-The%20scenario%20your)
    pub fn set_scenario(mut self, scenario: Scenario) -> Self {
        self.scenario = scenario;
        self
    }

    /// Open link when notification is clicked.
    /// # Examples
    /// ```
    /// use win_toast_notify::WinToastNotify;
    /// 
    /// WinToastNotify::new()
    ///     .set_title("Click me")
    ///     .set_open("https://www.google.com/")    // "C:/Windows/Web/Screen/img104.jpg"
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    pub fn set_open(mut self, url_or_path: &str) -> Self {
        self.open = Some(escape_str_attribute(url_or_path.trim()).into_owned());
        self
    }

    /// Set the title of the notification.
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = Some(escape_str_attribute(title).into_owned());
        self
    }

    /// Set the messages of the notification.
    /// 
    /// # Note
    /// Only supports adding two messages, but line breaks are allowed in the message content.
    /// # Examples
    /// ```
    /// use win_toast_notify::WinToastNotify;
    /// 
    /// WinToastNotify::new()
    ///     .set_title("Title")
    ///     .set_messages(vec![
    ///         "Mysterious Code",
    ///         "Infinitely Recursive Code"
    ///     ])
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// ```
    pub fn set_messages(mut self, messages: Vec<&str>) -> Self {
        self.messages = Some(Box::new(messages.iter().map(|t| escape_str_attribute(t).into_owned())).collect());
        self
    }

    /// Set the notification logo and specify whether to crop it into a circle.
    pub fn set_logo(mut self, path: &str, hint_crop: CropCircle) -> Self {
        self.logo = Some(escape_str_attribute(path.trim()).into_owned());
        self.logo_circle = hint_crop;
        self
    }

    /// Set the notification image and its position.
    pub fn set_image(mut self, path: &str, position: ImagePlacement) -> Self {
        self.image = Some(escape_str_attribute(path.trim()).into_owned());
        self.image_placement = position;
        self
    }

    /// Set the actions.
    /// # Example
    /// ```
    /// // Add two buttons to the notification.
    /// use win_toast_notify::{WinToastNotify, Action, ActivationType};
    /// 
    /// WinToastNotify::new()
    ///     .set_actions(vec![
    ///         Action {
    ///             activation_type: ActivationType::Protocol,
    ///             action_content: "Url".to_string(),
    ///             arguments: "https://www.google.com/".to_string(),
    ///             image_url: None
    ///         },
    ///         Action {
    ///             activation_type: ActivationType::Protocol,
    ///             action_content: "File".to_string(),
    ///             arguments: r"C:\Windows\Web\Screen\img104.jpg".to_string(),
    ///             image_url: None
    ///         },
    ///         Action {
    ///             activation_type: ActivationType::Protocol,
    ///             action_content: "Folder".to_string(),
    ///             arguments: r"$env:USERPROFILE\Downloads".to_string(),   // PowerShell supports using environment variables
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

    /// Set Progress.
    /// 
    /// tag: Define a tag (and optionally a group) to uniquely identify the notification, in order update the notification data later;
    /// 
    /// To avoid notifications disappearing when progress is not completed, 
    /// it is recommended to set the notification's Scenario to incomingCall and the Audio to silent
    /// 
    /// # Example
    /// ```
    /// use win_toast_notify::{WinToastNotify, Scenario, Audio, Loop, ActivationType, Action};
    /// 
    /// fn main() {
    ///     let tag = "weekly-playlist";
    ///     let title = "Weekly playlist";
    ///     let mut status = String::from("Downloading...");
    ///     let mut value = 0.0;
    ///     let mut value_string = String::from("0/10 songs");
    /// 
    ///     WinToastNotify::new()
    ///         .set_scenario(Scenario::IncomingCall)
    ///         .set_title("Downloading your weekly playlist...")
    ///         .set_progress(tag, title, &status, value, &value_string)
    ///         .set_audio(Audio::Silent, Loop::False)
    ///         .set_open("https://www.baidu.com")
    ///         .set_actions(vec![
    ///             Action {
    ///                 activation_type: ActivationType::Protocol,
    ///                 action_content: "Open Downloaads Folder".to_string(),
    ///                 arguments: r"$env:USERPROFILE\Downloads".to_string(),   // PowerShell supports using environment variables
    ///                 image_url: None
    ///             },
    ///         ])
    ///         .show()
    ///         .expect("Failed to show toast notification");
    ///
    ///     for i in 1..=10 {
    ///         std::thread::sleep(std::time::Duration::from_secs(1));
    ///         value = i as f32 / 10.0;
    ///         if i != 10 {
    ///             value_string = format!("{}/10 songs", i);
    ///             WinToastNotify::progress_update(None, tag, value, &value_string).expect("Failed to update");
    ///         } else {
    ///             status = String::from("Completed");
    ///             value_string = String::from("10/10 songs");
    ///             WinToastNotify::progress_complete(None, tag, &status, &value_string).expect("Failed to complete");
    ///         };
    ///     };
    /// }    
    /// ```
    /// 
    pub fn set_progress(mut self, tag: &str, title: &str, status: &str, value: f32, value_string: &str) -> Self {
        self.progress = Some( Progress{
            tag: tag.to_string(),
            title: title.to_string(),
            status: status.to_string(),
            value,
            value_string: value_string.to_string(),
        });
        self
    }
    
    /// Update the notification progress for the specified APP ID and tag
    pub fn progress_update(
        app_id: Option<&str>,
        tag: &str,
        value: f32,
        value_string: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = String::new();
        write!(
            command,
            r#"
            $Dictionary = [System.Collections.Generic.Dictionary[String, String]]::New()
            $Dictionary.Add('progressValue', {})
            $Dictionary.Add('progressValueString', "{}")
            $NotificationData = [Windows.UI.Notifications.NotificationData]::New($Dictionary)
            $NotificationData.SequenceNumber = 2
            $AppId = '{}'
            $Notifier = [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]::CreateToastNotifier($AppId)
            $Notifier.Update($NotificationData, '{}')
            "#,
            value,
            value_string,
            match app_id{
                Some(app_id) => app_id,
                None => r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe",
            },
            tag,
        )?;

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

    pub fn progress_complete(
        app_id: Option<&str>,
        tag: &str,
        status: &str,
        value_string: &str
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = String::new();
        write!(
            command,
            r#"
            $Dictionary = [System.Collections.Generic.Dictionary[String, String]]::New()
            $Dictionary.Add('progressStatus', '{}')
            $Dictionary.Add('progressValue', 1)
            $Dictionary.Add('progressValueString', "{}")
            $NotificationData = [Windows.UI.Notifications.NotificationData]::New($Dictionary)
            $NotificationData.SequenceNumber = 2
            $AppId = '{}'
            $Notifier = [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]::CreateToastNotifier($AppId)
            $Notifier.Update($NotificationData, '{}')
            "#,
            status,
            value_string,
            match app_id {
                Some(app_id) => app_id,
                None => 
                    r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe",
            },
            tag,
        )?;
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
            ).into());
        }

        Ok(())
    }

    /// Set the notification sound and whether the sound should loop.
    /// 
    /// Default is [Audio::WinDefault](enum.Audio.html)
    /// 
    /// # Example
    /// ```
    /// use win_toast_notify::{WinToastNotify, Audio, Loop};
    /// 
    /// // Use system audio and loop it.
    /// WinToastNotify::new()
    ///     .set_audio(Audio::WinLoopingAlarm5, Loop::True)
    ///     .show()
    ///     .expect("Failed to show toast notification");
    /// 
    /// // Use other audio, but don't loop it.(Currently unable to play other audio sources for unknown reasons)
    /// // WinToastNotify::new()
    /// //    .set_audio(Audio::From(r"C:\Windows\Media\Ring05.wav"), Loop::False)
    /// //    .show()
    /// //    .expect("Failed to show toast notification");
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
            <toast{}{}{}>
                <visual>
                    <binding template="ToastGeneric">
                        {}
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
            match &self.open {
                Some(url_or_path) => format!(r#" activationType="protocol" launch="{}""#, url_or_path),
                None => String::new(),
            },
            match &self.duration {
                Duration::Short => "",
                Duration::Long => " duration=\"long\"",
            },
            match &self.scenario {
                Scenario::None => "",
                Scenario::Reminder => " scenario=\"reminder\"",
                Scenario::Alarm => " scenario=\"alarm\"",
                Scenario::IncomingCall => " scenario=\"incomingCall\"",
                Scenario::Urgent => " scenario=\"urgent\"",
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
            match &self.progress {
                Some(_) => format!(
                    r#"
                    <progress
                        title="{{progressTitle}}"
                        value="{{progressValue}}"
                        valueStringOverride="{{progressValueString}}"
                        status="{{progressStatus}}"/>
                    "#
                ),
                None => String::new(),
            },
            match &self.actions {
                Some(actions) => actions.iter().fold(String::new(), |acc, action| {
                    format!(
                        "{}\n<action content=\"{}\" activationType=\"{}\" arguments=\"{}\" {} />",
                        acc,
                        escape_str_attribute(&action.action_content).into_owned(),
                        action.activation_type.as_str(),
                        escape_str_attribute(&action.arguments).into_owned(),
                        action.image_url.clone().map_or_else(
                            || String::new(),
                            |url| format!("imageUri=\"{}\"", escape_str_attribute(url.trim()).into_owned())),
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
            {}
            "#,
            match &self.app_id {
                Some(id) => id,
                None =>
                    r"{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\WindowsPowerShell\v1.0\powershell.exe",
            },
            match &self.progress {
                Some(progress) => {format!(
                        "
                        $ToastNotification = [Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications, ContentType = WindowsRuntime]::New($XmlDocument)
                        $ToastNotification.Tag = '{}'
                        $Dictionary = [System.Collections.Generic.Dictionary[String, String]]::New()
                        $Dictionary.Add('progressTitle', '{}')
                        $Dictionary.Add('progressValue', '{}')
                        $Dictionary.Add('progressValueString', '{}')
                        $Dictionary.Add('progressStatus', '{}')
                        $ToastNotification.Data = [Windows.UI.Notifications.NotificationData]::New($Dictionary)
                        $ToastNotification.Data.SequenceNumber = 1
                        [Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]::CreateToastNotifier($AppId).Show($ToastNotification)
                        ",
                        progress.tag,
                        progress.title,
                        &progress.value,
                        &progress.value_string,
                        &progress.status
                    )
                },
                None => "[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime]::CreateToastNotifier($AppId).Show($XmlDocument)".into(),
            },
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
                escape_str_attribute(url.trim()).into_owned()
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
}

// The scenario your toast is used for, like an alarm or reminder.
pub enum Scenario {
    None,
    Reminder,
    Alarm,
    IncomingCall,
    Urgent,
}

pub struct Action {
    pub activation_type: ActivationType,
    pub action_content: String,
    pub arguments: String,
    pub image_url: Option<String>,
}

/// [Microsoft Docs about Button](https://learn.microsoft.com/en-us/uwp/schemas/tiles/toastschema/element-action)
pub enum ActivationType {
    Protocol, // 使用协议激活功能启动不同的应用程序
    System,
    Background, // 触发相应的后台任务，而不会中断用户
    Foreground, // 启动前台应用程序（默认值）
}

impl ActivationType {
    pub fn as_str(&self) -> &str {
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

pub struct Progress {
    pub tag: String,
    pub title: String,
    pub status: String,
    pub value: f32,
    pub value_string: String,
}

// System Audio
pub enum Audio {
    From(String),
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
    pub fn as_str(&self) -> &str {
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

// Audio Loop
pub enum Loop {
    True,
    False,
}