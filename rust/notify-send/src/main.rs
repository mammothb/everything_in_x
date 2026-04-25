use windows::UI::Notifications::ToastNotification;
use windows::core::{HSTRING, Result};
use windows::{Data::Xml::Dom::XmlDocument, UI::Notifications::ToastNotificationManager};
use xml::escape::escape_str_attribute;

/// Use PowerShell's AppUserModelID.
pub const POWERSHELL_APP_ID: &str =
    "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}\\WindowsPowerShell\\v1.0\\powershell.exe";

struct MyToast {
    title: String,
    message: String,
}

impl MyToast {
    pub fn new(title: &str, message: &str) -> Self {
        Self {
            title: format!(r#"<text id="1">{}</text>"#, escape_str_attribute(title)),
            message: format!(r#"<text id="2">{}</text>"#, escape_str_attribute(message)),
        }
    }

    /// Display the toast on the screen
    pub fn show(&self) -> Result<()> {
        let toast_template = self.create_template()?;
        let toast_notifier =
            ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(POWERSHELL_APP_ID))?;

        // Show the toast.
        let result = toast_notifier.Show(&toast_template);
        std::thread::sleep(std::time::Duration::from_millis(10));
        result
    }

    fn create_template(&self) -> Result<ToastNotification> {
        //using this to get an instance of XmlDocument
        let toast_xml = XmlDocument::new()?;
        toast_xml.LoadXml(&HSTRING::from(format!(
            "<toast>
                <visual>
                    <binding template=\"ToastGeneric\">
                    {}{}
                    </binding>
                </visual>
            </toast>",
            self.title, self.message,
        )))?;
        // Create the toast
        ToastNotification::CreateToastNotification(&toast_xml)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: notify-send <title> <message>");
        std::process::exit(1);
    }
    MyToast::new(&args[1], &args[2])
        .show()
        .expect("unable to toast");
}
