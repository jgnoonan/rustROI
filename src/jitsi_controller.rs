use std::error::Error;
use async_trait::async_trait;
use std::process::Command;
use tokio;

#[async_trait]
pub trait PhoneController {
    async fn dial(&self, number: &str) -> Result<(), Box<dyn Error>>;
    async fn answer_call(&self, _call_id: &str) -> Result<(), Box<dyn Error>>;
    async fn hangup_call(&self, _call_id: &str) -> Result<(), Box<dyn Error>>;
    async fn send_dtmf(&self, _call_id: &str, digits: &str) -> Result<(), Box<dyn Error>>;
}

pub struct JitsiController;

impl JitsiController {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(JitsiController)
    }

    fn run_apple_script(&self, script: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }

        Ok(())
    }

    fn ensure_jitsi_running(&self) -> Result<(), Box<dyn Error>> {
        let script = r#"
        tell application "System Events"
            if not (exists process "Jitsi Meet") then
                tell application "Jitsi Meet" to activate
                delay 3
            end if
            tell application "Jitsi Meet" to activate
            delay 1
        end tell
        "#;
        self.run_apple_script(script)
    }

    fn open_jitsi_url(&self, number: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("https://meet.jit.si/{}", number);
        let script = format!(
            r#"
            tell application "System Events"
                tell process "Jitsi Meet"
                    set frontmost to true
                    delay 1
                    keystroke "l" using {{command down}}
                    delay 1
                    keystroke "{}"
                    delay 1
                    keystroke return
                end tell
            end tell
            "#,
            url
        );
        self.run_apple_script(&script)
    }
}

#[async_trait]
impl PhoneController for JitsiController {
    async fn dial(&self, number: &str) -> Result<(), Box<dyn Error>> {
        println!("Dialing {} via Jitsi Meet...", number);
        self.ensure_jitsi_running()?;
        
        // Wait for Jitsi to fully launch
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        self.open_jitsi_url(number)?;
        Ok(())
    }

    async fn answer_call(&self, _call_id: &str) -> Result<(), Box<dyn Error>> {
        let script = r#"
            tell application "Jitsi Meet"
                activate
                tell application "System Events"
                    keystroke "a" using {command down}
                end tell
            end tell
        "#;
        self.run_apple_script(script)
    }

    async fn hangup_call(&self, _call_id: &str) -> Result<(), Box<dyn Error>> {
        let script = r#"
            tell application "Jitsi Meet"
                activate
                tell application "System Events"
                    keystroke "h" using {command down}
                end tell
            end tell
        "#;
        self.run_apple_script(script)
    }

    async fn send_dtmf(&self, _call_id: &str, digits: &str) -> Result<(), Box<dyn Error>> {
        let script = format!(
            r#"
            tell application "Jitsi Meet"
                activate
                tell application "System Events"
                    keystroke "{}"
                end tell
            end tell
            "#,
            digits
        );
        self.run_apple_script(&script)
    }
}
