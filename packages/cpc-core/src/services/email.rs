use anyhow::{anyhow, Result};
use std::collections::HashMap;

/// Email service for sending notifications and passwordless login tokens
pub struct EmailService {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
}

/// Email template for passwordless login
#[derive(Debug, Clone)]
pub struct PasswordlessEmail {
    pub to_email: String,
    pub token: String,
    pub user_name: String,
}

impl EmailService {
    /// Creates a new email service instance
    pub fn new() -> Result<Self> {
        let smtp_server = std::env::var("SMTP_SERVER")
            .unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let smtp_username = std::env::var("SMTP_USERNAME")
            .map_err(|_| anyhow!("SMTP_USERNAME not configured"))?;
        let smtp_password = std::env::var("SMTP_PASSWORD")
            .map_err(|_| anyhow!("SMTP_PASSWORD not configured"))?;
        let from_email = std::env::var("FROM_EMAIL")
            .unwrap_or_else(|_| "noreply@cpc.coop".to_string());
        
        Ok(Self {
            smtp_server,
            smtp_username,
            smtp_password,
            from_email,
        })
    }
    
    /// Sends a passwordless login email with the token
    pub async fn send_passwordless_email(&self, email: PasswordlessEmail) -> Result<()> {
        let subject = "Your CPC Login Link";
        let body = format!(
            r#"
            Hi {name},
            
            Here's your secure login link for CPC:
            
            https://cpc.coop/auth/passwordless?token={token}&email={email}
            
            This link will expire in 15 minutes. If you didn't request this, please ignore this email.
            
            Best regards,
            The CPC Team
            "#,
            name = email.user_name,
            token = email.token,
            email = email.to_email
        );
        
        self.send_email(&email.to_email, subject, &body).await
    }
    
    /// Sends a generic email
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        // In a production environment, this would use a proper SMTP client
        // For now, we'll log the email content
        tracing::info!(
            "Sending email:\nTo: {}\nSubject: {}\nBody:\n{}",
            to,
            subject,
            body
        );
        
        // Simulate successful email sending
        Ok(())
    }
    
    /// Checks if email service is properly configured
    pub fn is_configured(&self) -> bool {
        !self.smtp_username.is_empty() && !self.smtp_password.is_empty()
    }
}

/// Mock email service for testing
pub struct MockEmailService;

impl MockEmailService {
    pub fn new() -> EmailService {
        EmailService {
            smtp_server: "mock".to_string(),
            smtp_username: "mock".to_string(),
            smtp_password: "mock".to_string(),
            from_email: "mock@example.com".to_string(),
        }
    }
}