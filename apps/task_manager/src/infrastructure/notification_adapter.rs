use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{Task, Reminder, Result};

#[cfg(target_os = "linux")]
use notify_rust::{Notification, Timeout};

#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "windows")]
use std::process::Command;

pub struct DesktopNotificationAdapter;

impl DesktopNotificationAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl crate::application::ports::NotificationService for DesktopNotificationAdapter {
    async fn send_notification(&self, title: &str, message: &str, task_id: Uuid) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            Notification::new()
                .summary(title)
                .body(message)
                .appname("CPC Task Manager")
                .timeout(Timeout::Milliseconds(5000))
                .show()
                .map_err(|e| crate::TaskError::NotificationError(e.to_string()))?;
        }

        #[cfg(target_os = "macos")]
        {
            let script = format!(
                r#"
                osascript -e 'display notification "{}" with title "CPC Task Manager" subtitle "{}"'
                "#,
                message.replace('"', "\\\""),
                title.replace('"', "\\\"")
            );
            
            Command::new("sh")
                .arg("-c")
                .arg(&script)
                .output()
                .map_err(|e| crate::TaskError::NotificationError(e.to_string()))?;
        }

        #[cfg(target_os = "windows")]
        {
            let script = format!(
                r#"
                Add-Type -AssemblyName System.Windows.Forms
                [System.Windows.Forms.MessageBox]::Show("{}", "CPC Task Manager - {}")
                "#,
                message.replace('"', "\\\""),
                title
            );
            
            Command::new("powershell")
                .arg("-Command")
                .arg(&script)
                .output()
                .map_err(|e| crate::TaskError::NotificationError(e.to_string()))?;
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            tracing::info!("Notification: {} - {}", title, message);
        }

        Ok(())
    }

    async fn schedule_reminder(&self, reminder: &Reminder, task: &Task) -> Result<()> {
        // For desktop notifications, we don't need to schedule anything
        // The reminder checking service will handle triggering
        tracing::info!(
            "Scheduled reminder for task {} at {}",
            task.title,
            reminder.remind_at
        );
        Ok(())
    }
}

pub struct MobileNotificationAdapter {
    #[cfg(target_os = "android")]
    jni_env: jni::JNIEnv<'static>,
    #[cfg(target_os = "android")]
    activity: jni::objects::JObject<'static>,
}

#[cfg(target_os = "android")]
impl MobileNotificationAdapter {
    pub fn new(env: jni::JNIEnv<'static>, activity: jni::objects::JObject<'static>) -> Self {
        Self {
            jni_env: env,
            activity,
        }
    }

    fn show_notification(&self, title: &str, message: &str) -> Result<()> {
        use jni::objects::{JClass, JString};
        use jni::sys::jstring;

        let cls = self.jni_env.find_class("android/content/Context")?;
        let notification_service = self.jni_env.get_static_field(
            cls,
            "NOTIFICATION_SERVICE",
            "Ljava/lang/String;"
        )?;

        // TODO: Implement proper Android notification
        tracing::info!("Mobile notification: {} - {}", title, message);
        Ok(())
    }
}

#[async_trait]
#[cfg(target_os = "android")]
impl crate::application::ports::NotificationService for MobileNotificationAdapter {
    async fn send_notification(&self, title: &str, message: &str, task_id: Uuid) -> Result<()> {
        self.show_notification(title, message)
    }

    async fn schedule_reminder(&self, reminder: &Reminder, task: &Task) -> Result<()> {
        // TODO: Implement proper Android alarm scheduling
        tracing::info!(
            "Scheduled mobile reminder for task {} at {}",
            task.title,
            reminder.remind_at
        );
        Ok(())
    }
}