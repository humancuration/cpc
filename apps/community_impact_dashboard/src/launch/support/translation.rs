//! Translation support for multilingual communities in the Unified Community Impact Dashboard
//!
//! This module provides translation services and localization support to ensure
//! the dashboard is accessible to multilingual community members.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Translation support system for multilingual communities
pub struct TranslationSupport {
    translations: HashMap<String, HashMap<String, String>>, // language -> key -> translation
    supported_languages: Vec<LanguageInfo>,
    translation_requests: Vec<TranslationRequest>,
    community_translators: HashMap<String, CommunityTranslator>,
    translation_memory: HashMap<String, TranslationMemoryEntry>,
    localization_settings: LocalizationSettings,
}

impl TranslationSupport {
    /// Create a new translation support system
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
            supported_languages: Vec::new(),
            translation_requests: Vec::new(),
            community_translators: HashMap::new(),
            translation_memory: HashMap::new(),
            localization_settings: LocalizationSettings::default(),
        }
    }

    /// Add a supported language
    pub fn add_supported_language(&mut self, language: LanguageInfo) {
        self.supported_languages.push(language);
        info!("Added supported language: {}", language.code);
    }

    /// Add translations for a language
    pub fn add_translations(&mut self, language_code: &str, translations: HashMap<String, String>) {
        self.translations.entry(language_code.to_string()).or_insert_with(HashMap::new).extend(translations);
        info!("Added translations for language: {}", language_code);
    }

    /// Get translation for a key in a specific language
    pub fn get_translation(&self, language_code: &str, key: &str) -> Option<&String> {
        self.translations.get(language_code)?.get(key)
    }

    /// Translate text using available translations
    pub fn translate(&self, text: &str, target_language: &str) -> Option<String> {
        // First check if we have a direct translation
        if let Some(lang_translations) = self.translations.get(target_language) {
            for (key, translation) in lang_translations {
                if key == text {
                    return Some(translation.clone());
                }
            }
        }
        
        // Check translation memory for similar texts
        for (_, entry) in &self.translation_memory {
            if entry.source_text == text && entry.target_language == target_language {
                return Some(entry.translated_text.clone());
            }
        }
        
        None
    }

    /// Request translation for content
    pub fn request_translation(&mut self, request: TranslationRequest) -> Uuid {
        let request_id = request.id;
        self.translation_requests.push(request);
        info!("Requested translation: {}", request_id);
        request_id
    }

    /// Register a community translator
    pub fn register_translator(&mut self, translator: CommunityTranslator) {
        self.community_translators.insert(translator.user_id.clone(), translator);
        info!("Registered community translator: {}", translator.user_id);
    }

    /// Assign translation request to a translator
    pub fn assign_translation_request(&mut self, request_id: Uuid, translator_id: String) -> Result<(), TranslationError> {
        let request = self.translation_requests.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or(TranslationError::RequestNotFound(request_id))?;

        request.assigned_translator = Some(translator_id.clone());
        request.status = TranslationStatus::Assigned;
        request.assigned_at = Some(Utc::now());
        
        // Update translator stats
        if let Some(translator) = self.community_translators.get_mut(&translator_id) {
            translator.current_assignments += 1;
        }
        
        info!("Assigned translation request {} to translator {}", request_id, translator_id);
        Ok(())
    }

    /// Submit completed translation
    pub fn submit_translation(&mut self, submission: TranslationSubmission) -> Result<(), TranslationError> {
        let request = self.translation_requests.iter_mut()
            .find(|r| r.id == submission.request_id)
            .ok_or(TranslationError::RequestNotFound(submission.request_id))?;

        request.status = TranslationStatus::Completed;
        request.completed_at = Some(Utc::now());
        request.translation_result = Some(submission.translation.clone());
        
        // Add to translation memory
        let memory_entry = TranslationMemoryEntry::new(
            submission.source_text,
            submission.translation,
            submission.source_language,
            submission.target_language,
        );
        self.translation_memory.insert(memory_entry.id.to_string(), memory_entry);
        
        // Update translator stats
        if let Some(translator_id) = &request.assigned_translator {
            if let Some(translator) = self.community_translators.get_mut(translator_id) {
                translator.completed_assignments += 1;
                translator.current_assignments -= 1;
                translator.total_words_translated += submission.word_count;
            }
        }
        
        info!("Submitted translation for request: {}", submission.request_id);
        Ok(())
    }

    /// Get translation requests by status
    pub fn get_requests_by_status(&self, status: TranslationStatus) -> Vec<&TranslationRequest> {
        self.translation_requests.iter()
            .filter(|r| r.status == status)
            .collect()
    }

    /// Get translation requests by language pair
    pub fn get_requests_by_language_pair(&self, source_lang: &str, target_lang: &str) -> Vec<&TranslationRequest> {
        self.translation_requests.iter()
            .filter(|r| r.source_language == source_lang && r.target_language == target_lang)
            .collect()
    }

    /// Get translation requests assigned to a translator
    pub fn get_translator_requests(&self, translator_id: &str) -> Vec<&TranslationRequest> {
        self.translation_requests.iter()
            .filter(|r| r.assigned_translator.as_deref() == Some(translator_id))
            .collect()
    }

    /// Get community translators by language pair
    pub fn get_translators_by_language_pair(&self, source_lang: &str, target_lang: &str) -> Vec<&CommunityTranslator> {
        self.community_translators.values()
            .filter(|t| t.language_pairs.contains(&(source_lang.to_string(), target_lang.to_string())))
            .collect()
    }

    /// Add translation to memory for future use
    pub fn add_to_translation_memory(&mut self, entry: TranslationMemoryEntry) {
        self.translation_memory.insert(entry.id.to_string(), entry);
        info!("Added entry to translation memory");
    }

    /// Get translation memory entries by language pair
    pub fn get_memory_entries_by_language_pair(&self, source_lang: &str, target_lang: &str) -> Vec<&TranslationMemoryEntry> {
        self.translation_memory.values()
            .filter(|e| e.source_language == source_lang && e.target_language == target_lang)
            .collect()
    }

    /// Get translator statistics
    pub fn get_translator_stats(&self, translator_id: &str) -> Option<&CommunityTranslator> {
        self.community_translators.get(translator_id)
    }

    /// Get top translators by completed assignments
    pub fn get_top_translators(&self, limit: usize) -> Vec<(&String, &CommunityTranslator)> {
        let mut translators: Vec<(&String, &CommunityTranslator)> = self.community_translators.iter().collect();
        translators.sort_by(|a, b| b.1.completed_assignments.cmp(&a.1.completed_assignments));
        translators.truncate(limit);
        translators
    }

    /// Get translation statistics
    pub fn get_statistics(&self) -> TranslationStatistics {
        let total_requests = self.translation_requests.len();
        let completed_requests = self.translation_requests.iter()
            .filter(|r| r.status == TranslationStatus::Completed)
            .count();
        
        let total_translators = self.community_translators.len();
        let total_languages = self.supported_languages.len();
        
        let total_memory_entries = self.translation_memory.len();
        
        TranslationStatistics {
            total_requests,
            completed_requests,
            pending_requests: total_requests - completed_requests,
            total_translators,
            total_languages,
            total_memory_entries,
        }
    }

    /// Generate translation support report
    pub fn generate_translation_report(&self) -> TranslationReport {
        let stats = self.get_statistics();
        let top_translators = self.get_top_translators(5);
        let language_coverage: HashMap<String, usize> = self.translation_requests.iter()
            .fold(HashMap::new(), |mut acc, request| {
                *acc.entry(request.target_language.clone()).or_insert(0) += 1;
                acc
            });
        
        TranslationReport {
            generated_at: Utc::now(),
            statistics: stats,
            top_translators: top_translators.into_iter().map(|(id, translator)| (id.clone(), translator.clone())).collect(),
            language_coverage,
        }
    }

    /// Set localization settings
    pub fn set_localization_settings(&mut self, settings: LocalizationSettings) {
        self.localization_settings = settings;
        info!("Updated localization settings");
    }

    /// Get localization settings
    pub fn get_localization_settings(&self) -> &LocalizationSettings {
        &self.localization_settings
    }

    /// Get supported languages
    pub fn get_supported_languages(&self) -> &Vec<LanguageInfo> {
        &self.supported_languages
    }

    /// Check if a language is supported
    pub fn is_language_supported(&self, language_code: &str) -> bool {
        self.supported_languages.iter().any(|lang| lang.code == language_code)
    }
}

/// Language information
#[derive(Debug, Clone)]
pub struct LanguageInfo {
    pub code: String, // ISO 639-1 code (e.g., "en", "es", "fr")
    pub name: String,
    pub native_name: String,
    pub direction: TextDirection,
    pub supported_scripts: Vec<String>,
}

impl LanguageInfo {
    /// Create new language information
    pub fn new(code: String, name: String, native_name: String, direction: TextDirection, supported_scripts: Vec<String>) -> Self {
        Self {
            code,
            name,
            native_name,
            direction,
            supported_scripts,
        }
    }
}

/// Text direction for language
#[derive(Debug, Clone)]
pub enum TextDirection {
    LeftToRight,
    RightToLeft,
}

/// Translation request from community members
#[derive(Debug, Clone)]
pub struct TranslationRequest {
    pub id: Uuid,
    pub requester: String,
    pub content: String,
    pub content_type: ContentType,
    pub source_language: String,
    pub target_language: String,
    pub priority: TranslationPriority,
    pub status: TranslationStatus,
    pub submitted_at: DateTime<Utc>,
    pub assigned_translator: Option<String>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub translation_result: Option<String>,
    pub word_count: usize,
    pub character_count: usize,
    pub tags: Vec<String>,
}

impl TranslationRequest {
    /// Create a new translation request
    pub fn new(
        requester: String,
        content: String,
        content_type: ContentType,
        source_language: String,
        target_language: String,
        priority: TranslationPriority,
    ) -> Self {
        let word_count = content.split_whitespace().count();
        let character_count = content.chars().count();
        
        Self {
            id: Uuid::new_v4(),
            requester,
            content,
            content_type,
            source_language,
            target_language,
            priority,
            status: TranslationStatus::Pending,
            submitted_at: Utc::now(),
            assigned_translator: None,
            assigned_at: None,
            completed_at: None,
            translation_result: None,
            word_count,
            character_count,
            tags: Vec::new(),
        }
    }

    /// Add tags to the translation request
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
    }
}

/// Translation submission from translators
#[derive(Debug, Clone)]
pub struct TranslationSubmission {
    pub request_id: Uuid,
    pub translator_id: String,
    pub source_text: String,
    pub translation: String,
    pub source_language: String,
    pub target_language: String,
    pub word_count: usize,
    pub submitted_at: DateTime<Utc>,
    pub quality_score: Option<f64>,
}

impl TranslationSubmission {
    /// Create a new translation submission
    pub fn new(
        request_id: Uuid,
        translator_id: String,
        source_text: String,
        translation: String,
        source_language: String,
        target_language: String,
    ) -> Self {
        let word_count = translation.split_whitespace().count();
        
        Self {
            request_id,
            translator_id,
            source_text,
            translation,
            source_language,
            target_language,
            word_count,
            submitted_at: Utc::now(),
            quality_score: None,
        }
    }
}

/// Community translator
#[derive(Debug, Clone)]
pub struct CommunityTranslator {
    pub user_id: String,
    pub name: String,
    pub language_pairs: Vec<(String, String)>, // (source, target) language pairs
    pub fluency_levels: HashMap<String, FluencyLevel>,
    pub registered_at: DateTime<Utc>,
    pub current_assignments: usize,
    pub completed_assignments: usize,
    pub total_words_translated: usize,
    pub average_quality_score: Option<f64>,
}

impl CommunityTranslator {
    /// Create a new community translator
    pub fn new(
        user_id: String,
        name: String,
        language_pairs: Vec<(String, String)>,
        fluency_levels: HashMap<String, FluencyLevel>,
    ) -> Self {
        Self {
            user_id,
            name,
            language_pairs,
            fluency_levels,
            registered_at: Utc::now(),
            current_assignments: 0,
            completed_assignments: 0,
            total_words_translated: 0,
            average_quality_score: None,
        }
    }
}

/// Translation memory entry for reuse
#[derive(Debug, Clone)]
pub struct TranslationMemoryEntry {
    pub id: Uuid,
    pub source_text: String,
    pub translated_text: String,
    pub source_language: String,
    pub target_language: String,
    pub created_at: DateTime<Utc>,
    pub usage_count: usize,
}

impl TranslationMemoryEntry {
    /// Create a new translation memory entry
    pub fn new(
        source_text: String,
        translated_text: String,
        source_language: String,
        target_language: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_text,
            translated_text,
            source_language,
            target_language,
            created_at: Utc::now(),
            usage_count: 0,
        }
    }

    /// Increment usage count
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }
}

/// Localization settings
#[derive(Debug, Clone)]
pub struct LocalizationSettings {
    pub default_language: String,
    pub fallback_languages: Vec<String>,
    pub auto_detect_language: bool,
    pub show_language_selector: bool,
    pub enable_community_translations: bool,
}

impl Default for LocalizationSettings {
    fn default() -> Self {
        Self {
            default_language: "en".to_string(),
            fallback_languages: vec!["en".to_string()],
            auto_detect_language: true,
            show_language_selector: true,
            enable_community_translations: true,
        }
    }
}

/// Types of content to translate
#[derive(Debug, Clone)]
pub enum ContentType {
    UserInterface,
    Documentation,
    CommunityStory,
    Feedback,
    Announcement,
    TrainingMaterial,
}

/// Priority levels for translation requests
#[derive(Debug, Clone)]
pub enum TranslationPriority {
    Low,
    Medium,
    High,
    Urgent,
}

/// Status of translation requests
#[derive(Debug, Clone, PartialEq)]
pub enum TranslationStatus {
    Pending,
    Assigned,
    InProgress,
    Completed,
    Rejected,
}

/// Fluency levels for translators
#[derive(Debug, Clone)]
pub enum FluencyLevel {
    Basic,
    Conversational,
    Professional,
    Native,
}

/// Statistics about translation support
#[derive(Debug, Clone)]
pub struct TranslationStatistics {
    pub total_requests: usize,
    pub completed_requests: usize,
    pub pending_requests: usize,
    pub total_translators: usize,
    pub total_languages: usize,
    pub total_memory_entries: usize,
}

/// Translation support report
#[derive(Debug, Clone)]
pub struct TranslationReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: TranslationStatistics,
    pub top_translators: Vec<(String, CommunityTranslator)>,
    pub language_coverage: HashMap<String, usize>,
}

/// Error types for translation support system
#[derive(Debug)]
pub enum TranslationError {
    RequestNotFound(Uuid),
    TranslatorNotFound(String),
    AssignmentError(String),
    SubmissionError(String),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationError::RequestNotFound(id) => write!(f, "Translation request not found: {}", id),
            TranslationError::TranslatorNotFound(id) => write!(f, "Translator not found: {}", id),
            TranslationError::AssignmentError(msg) => write!(f, "Assignment error: {}", msg),
            TranslationError::SubmissionError(msg) => write!(f, "Submission error: {}", msg),
        }
    }
}

impl std::error::Error for TranslationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_translation_support_initialization() {
        let translation = TranslationSupport::new();
        assert!(translation.translations.is_empty());
        assert!(translation.supported_languages.is_empty());
    }

    #[test]
    fn test_add_supported_language() {
        let mut translation = TranslationSupport::new();
        let language = LanguageInfo::new(
            "es".to_string(),
            "Spanish".to_string(),
            "Español".to_string(),
            TextDirection::LeftToRight,
            vec!["Latin".to_string()],
        );
        
        translation.add_supported_language(language);
        assert_eq!(translation.supported_languages.len(), 1);
        assert_eq!(translation.supported_languages[0].code, "es");
    }

    #[test]
    fn test_add_translations() {
        let mut translation = TranslationSupport::new();
        let mut translations = HashMap::new();
        translations.insert("welcome".to_string(), "Bienvenido".to_string());
        translations.insert("dashboard".to_string(), "Tablero".to_string());
        
        translation.add_translations("es", translations);
        assert!(translation.translations.contains_key("es"));
        assert_eq!(translation.translations["es"].len(), 2);
    }

    #[test]
    fn test_get_translation() {
        let mut translation = TranslationSupport::new();
        let mut translations = HashMap::new();
        translations.insert("welcome".to_string(), "Bienvenido".to_string());
        translation.add_translations("es", translations);
        
        let result = translation.get_translation("es", "welcome");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "Bienvenido");
    }

    #[test]
    fn test_request_translation() {
        let mut translation = TranslationSupport::new();
        let request = TranslationRequest::new(
            "user1".to_string(),
            "Welcome to our dashboard".to_string(),
            ContentType::UserInterface,
            "en".to_string(),
            "es".to_string(),
            TranslationPriority::Medium,
        );
        
        let request_id = translation.request_translation(request);
        assert!(!request_id.is_nil());
        assert_eq!(translation.translation_requests.len(), 1);
    }

    #[test]
    fn test_register_translator() {
        let mut translation = TranslationSupport::new();
        let mut fluency_levels = HashMap::new();
        fluency_levels.insert("es".to_string(), FluencyLevel::Native);
        
        let translator = CommunityTranslator::new(
            "translator1".to_string(),
            "Maria Garcia".to_string(),
            vec![("en".to_string(), "es".to_string())],
            fluency_levels,
        );
        
        translation.register_translator(translator);
        assert_eq!(translation.community_translators.len(), 1);
        assert!(translation.community_translators.contains_key("translator1"));
    }

    #[test]
    fn test_assign_translation_request() {
        let mut translation = TranslationSupport::new();
        
        let request = TranslationRequest::new(
            "user1".to_string(),
            "Welcome to our dashboard".to_string(),
            ContentType::UserInterface,
            "en".to_string(),
            "es".to_string(),
            TranslationPriority::Medium,
        );
        
        let request_id = translation.request_translation(request);
        
        let mut fluency_levels = HashMap::new();
        fluency_levels.insert("es".to_string(), FluencyLevel::Native);
        
        let translator = CommunityTranslator::new(
            "translator1".to_string(),
            "Maria Garcia".to_string(),
            vec![("en".to_string(), "es".to_string())],
            fluency_levels,
        );
        
        translation.register_translator(translator);
        
        let result = translation.assign_translation_request(request_id, "translator1".to_string());
        assert!(result.is_ok());
        
        let assigned_request = translation.translation_requests.iter().find(|r| r.id == request_id).unwrap();
        assert_eq!(assigned_request.status, TranslationStatus::Assigned);
        assert_eq!(assigned_request.assigned_translator, Some("translator1".to_string()));
    }

    #[test]
    fn test_submit_translation() {
        let mut translation = TranslationSupport::new();
        
        let request = TranslationRequest::new(
            "user1".to_string(),
            "Welcome to our dashboard".to_string(),
            ContentType::UserInterface,
            "en".to_string(),
            "es".to_string(),
            TranslationPriority::Medium,
        );
        
        let request_id = translation.request_translation(request);
        
        let submission = TranslationSubmission::new(
            request_id,
            "translator1".to_string(),
            "Welcome to our dashboard".to_string(),
            "Bienvenido a nuestro tablero".to_string(),
            "en".to_string(),
            "es".to_string(),
        );
        
        let result = translation.submit_translation(submission);
        assert!(result.is_ok());
        
        let completed_request = translation.translation_requests.iter().find(|r| r.id == request_id).unwrap();
        assert_eq!(completed_request.status, TranslationStatus::Completed);
        assert!(completed_request.translation_result.is_some());
    }

    #[test]
    fn test_get_requests_by_status() {
        let mut translation = TranslationSupport::new();
        
        let request = TranslationRequest::new(
            "user1".to_string(),
            "Welcome to our dashboard".to_string(),
            ContentType::UserInterface,
            "en".to_string(),
            "es".to_string(),
            TranslationPriority::Medium,
        );
        
        translation.request_translation(request);
        
        let pending_requests = translation.get_requests_by_status(TranslationStatus::Pending);
        assert_eq!(pending_requests.len(), 1);
    }

    #[test]
    fn test_get_translators_by_language_pair() {
        let mut translation = TranslationSupport::new();
        
        let mut fluency_levels = HashMap::new();
        fluency_levels.insert("es".to_string(), FluencyLevel::Native);
        
        let translator = CommunityTranslator::new(
            "translator1".to_string(),
            "Maria Garcia".to_string(),
            vec![("en".to_string(), "es".to_string())],
            fluency_levels,
        );
        
        translation.register_translator(translator);
        let translators = translation.get_translators_by_language_pair("en", "es");
        assert_eq!(translators.len(), 1);
    }

    #[test]
    fn test_add_to_translation_memory() {
        let mut translation = TranslationSupport::new();
        let entry = TranslationMemoryEntry::new(
            "Welcome".to_string(),
            "Bienvenido".to_string(),
            "en".to_string(),
            "es".to_string(),
        );
        
        translation.add_to_translation_memory(entry);
        assert_eq!(translation.translation_memory.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut translation = TranslationSupport::new();
        
        let request = TranslationRequest::new(
            "user1".to_string(),
            "Welcome to our dashboard".to_string(),
            ContentType::UserInterface,
            "en".to_string(),
            "es".to_string(),
            TranslationPriority::Medium,
        );
        
        translation.request_translation(request);
        
        let stats = translation.get_statistics();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.pending_requests, 1);
    }

    #[test]
    fn test_is_language_supported() {
        let mut translation = TranslationSupport::new();
        let language = LanguageInfo::new(
            "es".to_string(),
            "Spanish".to_string(),
            "Español".to_string(),
            TextDirection::LeftToRight,
            vec!["Latin".to_string()],
        );
        
        translation.add_supported_language(language);
        assert!(translation.is_language_supported("es"));
        assert!(!translation.is_language_supported("fr"));
    }
}