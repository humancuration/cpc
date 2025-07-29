use crate::domain::value_objects::{SalaryRange, EmploymentType, ApplicationStatus};
use crate::domain::errors::RecruitmentError;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employer {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    pub company_name_sharing: DataSharingSettings,
    pub company_description_sharing: DataSharingSettings,
    pub website_sharing: DataSharingSettings,
    pub industry_sharing: DataSharingSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
}

impl Employer {
    pub fn new(
        user_id: Uuid,
        company_name: String,
        company_description: Option<String>,
        website: Option<String>,
        industry: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Employer {
            id: Uuid::new_v4(),
            user_id,
            company_name,
            company_description,
            website,
            industry,
            company_name_sharing: DataSharingSettings::default(),
            company_description_sharing: DataSharingSettings::default(),
            website_sharing: DataSharingSettings::default(),
            industry_sharing: DataSharingSettings::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_company_info(
        &mut self,
        company_name: String,
        company_description: Option<String>,
        website: Option<String>,
        industry: Option<String>,
    ) -> Result<(), RecruitmentError> {
        self.company_name = company_name;
        self.company_description = company_description;
        self.website = website;
        self.industry = industry;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn update_sharing_settings(
        &mut self,
        field: &str,
        settings: DataSharingSettings,
    ) -> Result<(), RecruitmentError> {
        match field {
            "company_name" => self.company_name_sharing = settings,
            "company_description" => self.company_description_sharing = settings,
            "website" => self.website_sharing = settings,
            "industry" => self.industry_sharing = settings,
            _ => return Err(RecruitmentError::InvalidJobStatus(format!("Invalid field: {}", field))),
        }
        Ok(())
    }
    
    pub fn can_share_field_with(&self, field: &str, node_id: &Uuid) -> bool {
        let settings = match field {
            "company_name" => &self.company_name_sharing,
            "company_description" => &self.company_description_sharing,
            "website" => &self.website_sharing,
            "industry" => &self.industry_sharing,
            _ => return false,
        };
        
        settings.can_share_with(node_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub employer_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub is_remote: bool,
    pub salary_range: Option<SalaryRange>,
    pub employment_type: EmploymentType,
    pub status: JobStatus,
    pub title_sharing: DataSharingSettings,
    pub description_sharing: DataSharingSettings,
    pub location_sharing: DataSharingSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JobStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "closed")]
    Closed,
}

impl JobStatus {
    pub fn from_str(s: &str) -> Result<Self, RecruitmentError> {
        match s {
            "draft" => Ok(JobStatus::Draft),
            "published" => Ok(JobStatus::Published),
            "filled" => Ok(JobStatus::Filled),
            "closed" => Ok(JobStatus::Closed),
            _ => Err(RecruitmentError::InvalidJobStatus(s.to_string())),
        }
    }
}

impl Job {
    pub fn new(
        employer_id: Uuid,
        title: String,
        description: String,
        location: Option<String>,
        is_remote: bool,
        salary_range: Option<SalaryRange>,
        employment_type: EmploymentType,
    ) -> Self {
        let now = Utc::now();
        Job {
            id: Uuid::new_v4(),
            employer_id,
            title,
            description,
            location,
            is_remote,
            salary_range,
            employment_type,
            status: JobStatus::Draft,
            title_sharing: DataSharingSettings::default(),
            description_sharing: DataSharingSettings::default(),
            location_sharing: DataSharingSettings::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn publish(&mut self) -> Result<(), RecruitmentError> {
        if self.status == JobStatus::Draft {
            self.status = JobStatus::Published;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(RecruitmentError::InvalidJobStatus("Cannot publish job that is not in draft status".to_string()))
        }
    }
    
    pub fn close(&mut self) -> Result<(), RecruitmentError> {
        if self.status == JobStatus::Published {
            self.status = JobStatus::Closed;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(RecruitmentError::InvalidJobStatus("Cannot close job that is not published".to_string()))
        }
    }
    
    pub fn mark_as_filled(&mut self) -> Result<(), RecruitmentError> {
        if self.status == JobStatus::Published {
            self.status = JobStatus::Filled;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(RecruitmentError::InvalidJobStatus("Cannot mark as filled job that is not published".to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub headline: Option<String>,
    pub summary: Option<String>,
    pub location: Option<String>,
    pub is_open_to_work: bool,
    pub headline_sharing: DataSharingSettings,
    pub summary_sharing: DataSharingSettings,
    pub location_sharing: DataSharingSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Candidate {
    pub fn new(
        user_id: Uuid,
        headline: Option<String>,
        summary: Option<String>,
        location: Option<String>,
        is_open_to_work: bool,
    ) -> Self {
        let now = Utc::now();
        Candidate {
            id: Uuid::new_v4(),
            user_id,
            headline,
            summary,
            location,
            is_open_to_work,
            headline_sharing: DataSharingSettings::default(),
            summary_sharing: DataSharingSettings::default(),
            location_sharing: DataSharingSettings::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_profile(
        &mut self,
        headline: Option<String>,
        summary: Option<String>,
        location: Option<String>,
        is_open_to_work: bool,
    ) -> Result<(), RecruitmentError> {
        self.headline = headline;
        self.summary = summary;
        self.location = location;
        self.is_open_to_work = is_open_to_work;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    pub fn toggle_availability(&mut self) -> Result<(), RecruitmentError> {
        self.is_open_to_work = !self.is_open_to_work;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    pub id: Uuid,
    pub job_id: Uuid,
    pub candidate_id: Uuid,
    pub status: ApplicationStatus,
    pub cover_letter: Option<String>,
    pub cover_letter_sharing: DataSharingSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Application {
    pub fn new(
        job_id: Uuid,
        candidate_id: Uuid,
        cover_letter: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Application {
            id: Uuid::new_v4(),
            job_id,
            candidate_id,
            status: ApplicationStatus::Applied,
            cover_letter,
            cover_letter_sharing: DataSharingSettings::default(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_status(&mut self, status: ApplicationStatus) -> Result<(), RecruitmentError> {
        self.status = status;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub document_id: Uuid,
    pub parsed_content: Option<serde_json::Value>,
    pub parsed_content_sharing: DataSharingSettings,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSharingSettings {
    pub shared_with: Vec<Uuid>,
    pub visibility: SharingVisibility,
}

impl DataSharingSettings {
    pub fn new(shared_with: Vec<Uuid>, visibility: SharingVisibility) -> Self {
        Self {
            shared_with,
            visibility,
        }
    }
    
    pub fn can_share_with(&self, node_id: &Uuid) -> bool {
        match self.visibility {
            SharingVisibility::None => false,
            SharingVisibility::Federation => {
                // For federation, check if explicitly shared with this node or if it's public
                self.shared_with.contains(node_id)
            },
            SharingVisibility::Public => true,
        }
    }
    
    pub fn add_shared_node(&mut self, node_id: Uuid) {
        if !self.shared_with.contains(&node_id) {
            self.shared_with.push(node_id);
        }
    }
    
    pub fn remove_shared_node(&mut self, node_id: &Uuid) {
        self.shared_with.retain(|id| id != node_id);
    }
}

impl Default for DataSharingSettings {
    fn default() -> Self {
        Self {
            shared_with: Vec::new(),
            visibility: SharingVisibility::None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingVisibility {
    None,
    Federation,
    Public,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    
    #[test]
    fn test_employer_creation() {
        let user_id = Uuid::new_v4();
        let employer = Employer::new(
            user_id,
            "Test Company".to_string(),
            Some("A test company".to_string()),
            Some("https://test.com".to_string()),
            Some("Technology".to_string()),
        );
        
        assert_eq!(employer.user_id, user_id);
        assert_eq!(employer.company_name, "Test Company");
        assert_eq!(employer.company_description, Some("A test company".to_string()));
        assert_eq!(employer.website, Some("https://test.com".to_string()));
        assert_eq!(employer.industry, Some("Technology".to_string()));
    }
    
    #[test]
    fn test_employer_update() {
        let user_id = Uuid::new_v4();
        let mut employer = Employer::new(
            user_id,
            "Test Company".to_string(),
            None,
            None,
            None,
        );
        
        let result = employer.update_company_info(
            "Updated Company".to_string(),
            Some("Updated description".to_string()),
            Some("https://updated.com".to_string()),
            Some("Finance".to_string()),
        );
        
        assert!(result.is_ok());
        assert_eq!(employer.company_name, "Updated Company");
        assert_eq!(employer.company_description, Some("Updated description".to_string()));
        assert_eq!(employer.website, Some("https://updated.com".to_string()));
        assert_eq!(employer.industry, Some("Finance".to_string()));
    }
    
    #[test]
    fn test_job_creation() {
        let employer_id = Uuid::new_v4();
        let salary_range = SalaryRange::new(
            Some(Decimal::new(50000, 0)),
            Some(Decimal::new(80000, 0)),
        ).unwrap();
        
        let job = Job::new(
            employer_id,
            "Software Engineer".to_string(),
            "Develop software applications".to_string(),
            Some("San Francisco".to_string()),
            false,
            Some(salary_range),
            EmploymentType::FullTime,
        );
        
        assert_eq!(job.employer_id, employer_id);
        assert_eq!(job.title, "Software Engineer");
        assert_eq!(job.description, "Develop software applications");
        assert_eq!(job.location, Some("San Francisco".to_string()));
        assert_eq!(job.is_remote, false);
        assert_eq!(job.employment_type, EmploymentType::FullTime);
        assert_eq!(job.status, JobStatus::Draft);
    }
    
    #[test]
    fn test_job_publishing() {
        let employer_id = Uuid::new_v4();
        let mut job = Job::new(
            employer_id,
            "Software Engineer".to_string(),
            "Develop software applications".to_string(),
            Some("San Francisco".to_string()),
            false,
            None,
            EmploymentType::FullTime,
        );
        
        // Should be able to publish a draft job
        let result = job.publish();
        assert!(result.is_ok());
        assert_eq!(job.status, JobStatus::Published);
        
        // Should not be able to publish an already published job
        let result = job.publish();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_candidate_creation() {
        let user_id = Uuid::new_v4();
        let candidate = Candidate::new(
            user_id,
            Some("Software Developer".to_string()),
            Some("Experienced developer with 5 years of experience".to_string()),
            Some("New York".to_string()),
            true,
        );
        
        assert_eq!(candidate.user_id, user_id);
        assert_eq!(candidate.headline, Some("Software Developer".to_string()));
        assert_eq!(candidate.summary, Some("Experienced developer with 5 years of experience".to_string()));
        assert_eq!(candidate.location, Some("New York".to_string()));
        assert_eq!(candidate.is_open_to_work, true);
    }
    
    #[test]
    fn test_candidate_toggle_availability() {
        let user_id = Uuid::new_v4();
        let mut candidate = Candidate::new(
            user_id,
            None,
            None,
            None,
            true,
        );
        
        assert_eq!(candidate.is_open_to_work, true);
        
        let result = candidate.toggle_availability();
        assert!(result.is_ok());
        assert_eq!(candidate.is_open_to_work, false);
        
        let result = candidate.toggle_availability();
        assert!(result.is_ok());
        assert_eq!(candidate.is_open_to_work, true);
    }
    
    #[test]
    fn test_application_creation() {
        let job_id = Uuid::new_v4();
        let candidate_id = Uuid::new_v4();
        let application = Application::new(
            job_id,
            candidate_id,
            Some("I'm excited to apply for this position".to_string()),
        );
        
        assert_eq!(application.job_id, job_id);
        assert_eq!(application.candidate_id, candidate_id);
        assert_eq!(application.status, ApplicationStatus::Applied);
        assert_eq!(application.cover_letter, Some("I'm excited to apply for this position".to_string()));
    }
    
    #[test]
    fn test_application_status_update() {
        let job_id = Uuid::new_v4();
        let candidate_id = Uuid::new_v4();
        let mut application = Application::new(
            job_id,
            candidate_id,
            None,
        );
        
        assert_eq!(application.status, ApplicationStatus::Applied);
        
        let result = application.update_status(ApplicationStatus::Reviewed);
        assert!(result.is_ok());
        assert_eq!(application.status, ApplicationStatus::Reviewed);
    }
    
    #[test]
    fn test_data_sharing_settings() {
        let node_id = Uuid::new_v4();
        let mut settings = DataSharingSettings::new(vec![node_id], SharingVisibility::Federation);
        
        // Test can_share_with
        assert!(settings.can_share_with(&node_id));
        assert!(!settings.can_share_with(&Uuid::new_v4()));
        
        // Test add_shared_node
        let new_node_id = Uuid::new_v4();
        settings.add_shared_node(new_node_id);
        assert!(settings.can_share_with(&new_node_id));
        
        // Test remove_shared_node
        settings.remove_shared_node(&node_id);
        assert!(!settings.can_share_with(&node_id));
        
        // Test public visibility
        settings.visibility = SharingVisibility::Public;
        assert!(settings.can_share_with(&Uuid::new_v4()));
        
        // Test none visibility
        settings.visibility = SharingVisibility::None;
        assert!(!settings.can_share_with(&new_node_id));
    }
}