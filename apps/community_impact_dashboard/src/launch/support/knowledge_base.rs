//! Knowledge base with community-contributed solutions for the Unified Community Impact Dashboard
//!
//! This module provides a collaborative knowledge base where community members
//! can contribute solutions, share best practices, and learn from each other's experiences.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Knowledge base system for community solutions
pub struct KnowledgeBase {
    articles: Vec<KnowledgeArticle>,
    categories: HashMap<String, KnowledgeCategory>,
    contributor_stats: HashMap<String, ContributorStats>,
    article_ratings: HashMap<Uuid, Vec<ArticleRating>>,
    search_index: HashMap<String, Vec<Uuid>>,
}

impl KnowledgeBase {
    /// Create a new knowledge base system
    pub fn new() -> Self {
        Self {
            articles: Vec::new(),
            categories: HashMap::new(),
            contributor_stats: HashMap::new(),
            article_ratings: HashMap::new(),
            search_index: HashMap::new(),
        }
    }

    /// Create a knowledge category
    pub fn create_category(&mut self, category: KnowledgeCategory) -> String {
        let category_id = category.id.clone();
        self.categories.insert(category_id.clone(), category);
        info!("Created knowledge category: {}", category_id);
        category_id
    }

    /// Add a knowledge article
    pub fn add_article(&mut self, article: KnowledgeArticle) -> Uuid {
        let article_id = article.id;
        
        // Update contributor statistics
        let contributor = article.author.clone();
        let stats = self.contributor_stats.entry(contributor).or_insert_with(ContributorStats::new);
        stats.articles_contributed += 1;
        stats.total_contributions += 1;
        
        // Update search index
        self.index_article_content(&article);
        
        self.articles.push(article);
        info!("Added knowledge article: {}", article_id);
        article_id
    }

    /// Index article content for search
    fn index_article_content(&mut self, article: &KnowledgeArticle) {
        // Simple indexing - in a real implementation, this would be more sophisticated
        let words: Vec<&str> = article.title.split_whitespace().collect();
        for word in words {
            let word_lower = word.to_lowercase();
            self.search_index.entry(word_lower).or_insert_with(Vec::new).push(article.id);
        }
        
        let words: Vec<&str> = article.content.split_whitespace().collect();
        for word in words {
            let word_lower = word.to_lowercase();
            self.search_index.entry(word_lower).or_insert_with(Vec::new).push(article.id);
        }
    }

    /// Get articles by category
    pub fn get_articles_by_category(&self, category_id: &str) -> Vec<&KnowledgeArticle> {
        self.articles.iter()
            .filter(|a| a.category == category_id)
            .collect()
    }

    /// Get articles by tag
    pub fn get_articles_by_tag(&self, tag: &str) -> Vec<&KnowledgeArticle> {
        self.articles.iter()
            .filter(|a| a.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Search articles by keyword
    pub fn search_articles(&self, keyword: &str) -> Vec<&KnowledgeArticle> {
        let keyword_lower = keyword.to_lowercase();
        if let Some(article_ids) = self.search_index.get(&keyword_lower) {
            article_ids.iter()
                .filter_map(|id| self.articles.iter().find(|a| a.id == *id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Rate an article
    pub fn rate_article(&mut self, rating: ArticleRating) -> Result<(), KnowledgeBaseError> {
        let article = self.articles.iter()
            .find(|a| a.id == rating.article_id)
            .ok_or(KnowledgeBaseError::ArticleNotFound(rating.article_id))?;

        // Update article rating count
        let ratings = self.article_ratings.entry(rating.article_id).or_insert_with(Vec::new);
        ratings.push(rating);
        
        // Update contributor stats
        let stats = self.contributor_stats.entry(article.author.clone()).or_insert_with(ContributorStats::new);
        stats.total_ratings += 1;
        
        info!("Rated article: {}", rating.article_id);
        Ok(())
    }

    /// Get article rating statistics
    pub fn get_article_rating_stats(&self, article_id: Uuid) -> Option<ArticleRatingStats> {
        let ratings = self.article_ratings.get(&article_id)?;
        
        if ratings.is_empty() {
            return None;
        }
        
        let total_ratings = ratings.len() as f64;
        let average_rating = ratings.iter().map(|r| r.rating as f64).sum::<f64>() / total_ratings;
        
        let rating_distribution: HashMap<u8, usize> = ratings.iter()
            .fold(HashMap::new(), |mut acc, r| {
                *acc.entry(r.rating).or_insert(0) += 1;
                acc
            });
        
        Some(ArticleRatingStats {
            article_id,
            average_rating,
            total_ratings: ratings.len(),
            rating_distribution,
        })
    }

    /// Get top-rated articles
    pub fn get_top_rated_articles(&self, limit: usize) -> Vec<(&KnowledgeArticle, f64)> {
        let mut rated_articles: Vec<(&KnowledgeArticle, f64)> = self.articles.iter()
            .filter_map(|article| {
                if let Some(stats) = self.get_article_rating_stats(article.id) {
                    Some((article, stats.average_rating))
                } else {
                    None
                }
            })
            .collect();
        
        rated_articles.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        rated_articles.truncate(limit);
        rated_articles
    }

    /// Get articles by author
    pub fn get_articles_by_author(&self, author: &str) -> Vec<&KnowledgeArticle> {
        self.articles.iter()
            .filter(|a| a.author == author)
            .collect()
    }

    /// Get contributor statistics
    pub fn get_contributor_stats(&self, contributor: &str) -> Option<&ContributorStats> {
        self.contributor_stats.get(contributor)
    }

    /// Get top contributors
    pub fn get_top_contributors(&self, limit: usize) -> Vec<(&String, &ContributorStats)> {
        let mut contributors: Vec<(&String, &ContributorStats)> = self.contributor_stats.iter().collect();
        contributors.sort_by(|a, b| b.1.articles_contributed.cmp(&a.1.articles_contributed));
        contributors.truncate(limit);
        contributors
    }

    /// Update an existing article
    pub fn update_article(&mut self, article_id: Uuid, updates: ArticleUpdate) -> Result<(), KnowledgeBaseError> {
        let article = self.articles.iter_mut()
            .find(|a| a.id == article_id)
            .ok_or(KnowledgeBaseError::ArticleNotFound(article_id))?;

        if let Some(title) = updates.title {
            article.title = title;
        }
        
        if let Some(content) = updates.content {
            article.content = content;
        }
        
        if let Some(tags) = updates.tags {
            article.tags = tags;
        }
        
        article.updated_at = Utc::now();
        
        // Re-index the updated article
        self.index_article_content(article);
        
        info!("Updated article: {}", article_id);
        Ok(())
    }

    /// Delete an article
    pub fn delete_article(&mut self, article_id: Uuid) -> Result<(), KnowledgeBaseError> {
        let index = self.articles.iter().position(|a| a.id == article_id)
            .ok_or(KnowledgeBaseError::ArticleNotFound(article_id))?;
        
        let article = self.articles.remove(index);
        
        // Update contributor stats
        if let Some(stats) = self.contributor_stats.get_mut(&article.author) {
            stats.articles_contributed = stats.articles_contributed.saturating_sub(1);
        }
        
        // Remove ratings
        self.article_ratings.remove(&article_id);
        
        info!("Deleted article: {}", article_id);
        Ok(())
    }

    /// Get knowledge base statistics
    pub fn get_statistics(&self) -> KnowledgeBaseStats {
        let total_articles = self.articles.len();
        let total_categories = self.categories.len();
        let total_contributors = self.contributor_stats.len();
        let total_ratings: usize = self.article_ratings.values().map(|r| r.len()).sum();
        
        let top_category = self.articles.iter()
            .fold(HashMap::new(), |mut acc, article| {
                *acc.entry(article.category.clone()).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(category, _)| category);
        
        KnowledgeBaseStats {
            total_articles,
            total_categories,
            total_contributors,
            total_ratings,
            top_category,
        }
    }

    /// Generate knowledge base report
    pub fn generate_knowledge_report(&self) -> KnowledgeBaseReport {
        let stats = self.get_statistics();
        let top_contributors = self.get_top_contributors(5);
        let top_rated_articles = self.get_top_rated_articles(5);
        
        KnowledgeBaseReport {
            generated_at: Utc::now(),
            statistics: stats,
            top_contributors: top_contributors.into_iter().map(|(name, stats)| (name.clone(), stats.clone())).collect(),
            top_rated_articles: top_rated_articles.into_iter().map(|(article, rating)| (article.id, rating)).collect(),
        }
    }
}

/// Knowledge article contributed by community members
#[derive(Debug, Clone)]
pub struct KnowledgeArticle {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String, // Category ID
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub views: usize,
    pub is_community_verified: bool,
    pub related_articles: Vec<Uuid>,
}

impl KnowledgeArticle {
    /// Create a new knowledge article
    pub fn new(
        title: String,
        content: String,
        category: String,
        author: String,
        tags: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            category,
            author,
            created_at: now,
            updated_at: now,
            tags,
            views: 0,
            is_community_verified: false,
            related_articles: Vec::new(),
        }
    }

    /// Increment view count
    pub fn increment_views(&mut self) {
        self.views += 1;
    }

    /// Mark as community verified
    pub fn mark_verified(&mut self) {
        self.is_community_verified = true;
        self.updated_at = Utc::now();
    }

    /// Add related articles
    pub fn add_related_articles(&mut self, articles: Vec<Uuid>) {
        self.related_articles.extend(articles);
    }
}

/// Knowledge category for organizing articles
#[derive(Debug, Clone)]
pub struct KnowledgeCategory {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parent_category: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl KnowledgeCategory {
    /// Create a new knowledge category
    pub fn new(id: String, name: String, description: String, parent_category: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            parent_category,
            created_at: Utc::now(),
        }
    }
}

/// Rating for a knowledge article
#[derive(Debug, Clone)]
pub struct ArticleRating {
    pub id: Uuid,
    pub article_id: Uuid,
    pub rater: String,
    pub rating: u8, // 1-5 stars
    pub comment: Option<String>,
    pub rated_at: DateTime<Utc>,
}

impl ArticleRating {
    /// Create a new article rating
    pub fn new(article_id: Uuid, rater: String, rating: u8, comment: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            article_id,
            rater,
            rating,
            comment,
            rated_at: Utc::now(),
        }
    }
}

/// Statistics for article ratings
#[derive(Debug, Clone)]
pub struct ArticleRatingStats {
    pub article_id: Uuid,
    pub average_rating: f64,
    pub total_ratings: usize,
    pub rating_distribution: HashMap<u8, usize>,
}

/// Contributor statistics
#[derive(Debug, Clone)]
pub struct ContributorStats {
    pub articles_contributed: usize,
    pub total_contributions: usize,
    pub total_ratings: usize,
    pub average_rating: Option<f64>,
}

impl ContributorStats {
    /// Create new contributor statistics
    pub fn new() -> Self {
        Self {
            articles_contributed: 0,
            total_contributions: 0,
            total_ratings: 0,
            average_rating: None,
        }
    }
}

/// Updates for an existing article
#[derive(Debug, Clone)]
pub struct ArticleUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Knowledge base statistics
#[derive(Debug, Clone)]
pub struct KnowledgeBaseStats {
    pub total_articles: usize,
    pub total_categories: usize,
    pub total_contributors: usize,
    pub total_ratings: usize,
    pub top_category: Option<String>,
}

/// Knowledge base report
#[derive(Debug, Clone)]
pub struct KnowledgeBaseReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: KnowledgeBaseStats,
    pub top_contributors: Vec<(String, ContributorStats)>,
    pub top_rated_articles: Vec<(Uuid, f64)>,
}

/// Error types for knowledge base system
#[derive(Debug)]
pub enum KnowledgeBaseError {
    ArticleNotFound(Uuid),
    CategoryNotFound(String),
    UpdateError(String),
    DeleteError(String),
}

impl std::fmt::Display for KnowledgeBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KnowledgeBaseError::ArticleNotFound(id) => write!(f, "Article not found: {}", id),
            KnowledgeBaseError::CategoryNotFound(id) => write!(f, "Category not found: {}", id),
            KnowledgeBaseError::UpdateError(msg) => write!(f, "Update error: {}", msg),
            KnowledgeBaseError::DeleteError(msg) => write!(f, "Delete error: {}", msg),
        }
    }
}

impl std::error::Error for KnowledgeBaseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_base_initialization() {
        let kb = KnowledgeBase::new();
        assert!(kb.articles.is_empty());
        assert!(kb.categories.is_empty());
    }

    #[test]
    fn test_create_category() {
        let mut kb = KnowledgeBase::new();
        let category = KnowledgeCategory::new(
            "technical".to_string(),
            "Technical Issues".to_string(),
            "Solutions for technical problems".to_string(),
            None,
        );
        
        let category_id = kb.create_category(category);
        assert_eq!(category_id, "technical");
        assert!(kb.categories.contains_key("technical"));
    }

    #[test]
    fn test_add_article() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "How to Navigate the Dashboard".to_string(),
            "Detailed instructions for dashboard navigation".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "dashboard".to_string()],
        );
        
        let article_id = kb.add_article(article);
        assert!(!article_id.is_nil());
        assert_eq!(kb.articles.len(), 1);
        
        // Check contributor stats
        let stats = kb.get_contributor_stats("user1").unwrap();
        assert_eq!(stats.articles_contributed, 1);
    }

    #[test]
    fn test_search_articles() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        kb.add_article(article);
        let results = kb.search_articles("navigation");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_rate_article() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        let article_id = kb.add_article(article);
        let rating = ArticleRating::new(
            article_id,
            "user2".to_string(),
            5,
            Some("Very helpful guide".to_string()),
        );
        
        let result = kb.rate_article(rating);
        assert!(result.is_ok());
        
        let stats = kb.get_article_rating_stats(article_id).unwrap();
        assert_eq!(stats.total_ratings, 1);
        assert_eq!(stats.average_rating, 5.0);
    }

    #[test]
    fn test_get_articles_by_category() {
        let mut kb = KnowledgeBase::new();
        let category = KnowledgeCategory::new(
            "technical".to_string(),
            "Technical Issues".to_string(),
            "Solutions for technical problems".to_string(),
            None,
        );
        kb.create_category(category);
        
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        kb.add_article(article);
        let articles = kb.get_articles_by_category("technical");
        assert_eq!(articles.len(), 1);
    }

    #[test]
    fn test_get_articles_by_author() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        kb.add_article(article);
        let articles = kb.get_articles_by_author("user1");
        assert_eq!(articles.len(), 1);
    }

    #[test]
    fn test_update_article() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        let article_id = kb.add_article(article);
        let updates = ArticleUpdate {
            title: Some("Updated Dashboard Navigation Guide".to_string()),
            content: Some("Updated instructions for dashboard navigation".to_string()),
            tags: Some(vec!["navigation".to_string(), "updated".to_string()]),
        };
        
        let result = kb.update_article(article_id, updates);
        assert!(result.is_ok());
        
        let updated_article = kb.articles.iter().find(|a| a.id == article_id).unwrap();
        assert_eq!(updated_article.title, "Updated Dashboard Navigation Guide");
    }

    #[test]
    fn test_delete_article() {
        let mut kb = KnowledgeBase::new();
        let article = KnowledgeArticle::new(
            "Dashboard Navigation Guide".to_string(),
            "How to navigate the dashboard interface".to_string(),
            "technical".to_string(),
            "user1".to_string(),
            vec!["navigation".to_string(), "guide".to_string()],
        );
        
        let article_id = kb.add_article(article);
        let result = kb.delete_article(article_id);
        assert!(result.is_ok());
        assert_eq!(kb.articles.len(), 0);
        
        // Check contributor stats
        let stats = kb.get_contributor_stats("user1").unwrap();
        assert_eq!(stats.articles_contributed, 0);
    }
}