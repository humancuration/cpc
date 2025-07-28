use crate::domain::models::{Job, Candidate};
use crate::domain::errors::RecruitmentError;
use uuid::Uuid;
use std::collections::HashMap;

pub struct MatchingEngine;

impl MatchingEngine {
    pub fn new() -> Self {
        MatchingEngine
    }
    
    pub fn calculate_job_candidate_match(&self, job: &Job, candidate: &Candidate) -> Result<f64, RecruitmentError> {
        // Simple matching algorithm based on:
        // 1. Location match (remote work counts as match)
        // 2. Keywords in job title/description vs candidate headline/summary
        // 3. Employment type preference match
        
        let mut score = 0.0;
        
        // Location matching (30% of total score)
        if job.is_remote || 
           (job.location.is_some() && candidate.location.is_some() && 
            job.location.as_ref().unwrap() == candidate.location.as_ref().unwrap()) {
            score += 30.0;
        }
        
        // Keyword matching (50% of total score)
        let keyword_score = self.calculate_keyword_match(job, candidate)?;
        score += keyword_score * 0.5;
        
        // Employment type matching (20% of total score)
        // For simplicity, we'll assume all employment types are acceptable
        score += 20.0;
        
        Ok(score.min(100.0)) // Cap at 100%
    }
    
    fn calculate_keyword_match(&self, job: &Job, candidate: &Candidate) -> Result<f64, RecruitmentError> {
        // Extract keywords from job description and title
        let job_keywords = self.extract_keywords(&job.title, &job.description);
        
        // Extract keywords from candidate profile
        let candidate_keywords = self.extract_candidate_keywords(candidate);
        
        if job_keywords.is_empty() || candidate_keywords.is_empty() {
            return Ok(0.0);
        }
        
        // Calculate Jaccard similarity
        let intersection: usize = job_keywords.iter()
            .filter(|keyword| candidate_keywords.contains(*keyword))
            .count();
        
        let union: usize = job_keywords.iter()
            .chain(candidate_keywords.iter())
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        if union == 0 {
            Ok(0.0)
        } else {
            Ok((intersection as f64 / union as f64) * 100.0)
        }
    }
    
    fn extract_keywords(&self, title: &str, description: &str) -> Vec<String> {
        // Simple keyword extraction - in a real system, you would use NLP
        let content = format!("{} {}", title, description);
        let words: Vec<String> = content
            .split_whitespace()
            .map(|word| {
                word.trim_matches(|c: char| !c.is_alphabetic())
                    .to_lowercase()
            })
            .filter(|word| word.len() > 2) // Filter out short words
            .collect();
        
        // Remove common stop words
        let stop_words = vec![
            "the", "and", "for", "are", "but", "not", "you", "all", "can", "had", "her", "was", 
            "one", "our", "out", "day", "get", "has", "him", "his", "how", "its", "may", "new", 
            "now", "old", "see", "two", "who", "boy", "did", "man", "men", "put", "too", "use", 
            "any", "big", "end", "far", "got", "job", "law", "let", "lot", "low", "old", "own", 
            "set", "she", "try", "way", "win", "yes", "yet", "bit", "eat", "fit", "hit", "hot", 
            "ill", "inn", "joy", "lay", "led", "leg", "lie", "log", "map", "met", "net", "nod", 
            "nor", "oak", "pay", "per", "pin", "pop", "pot", "put", "raw", "red", "rid", "run", 
            "sat", "saw", "say", "sea", "she", "shy", "sin", "sit", "six", "sky", "son", "sun", 
            "ten", "tip", "top", "toy", "try", "two", "war", "wet", "win", "yes", "yet", "zip"
        ];
        
        words.into_iter()
            .filter(|word| !stop_words.contains(&word.as_str()))
            .collect()
    }
    
    fn extract_candidate_keywords(&self, candidate: &Candidate) -> Vec<String> {
        let mut content = String::new();
        
        if let Some(headline) = &candidate.headline {
            content.push_str(headline);
            content.push(' ');
        }
        
        if let Some(summary) = &candidate.summary {
            content.push_str(summary);
        }
        
        self.extract_keywords("", &content)
    }
    
    pub fn find_best_matches_for_job(&self, job: &Job, candidates: &[Candidate]) -> Result<Vec<(Uuid, f64)>, RecruitmentError> {
        let mut matches = Vec::new();
        
        for candidate in candidates {
            let score = self.calculate_job_candidate_match(job, candidate)?;
            if score > 80.0 { // Only include strong matches
                matches.push((candidate.id, score));
            }
        }
        
        // Sort by score descending
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(matches)
    }
    
    pub fn find_best_matches_for_candidate(&self, candidate: &Candidate, jobs: &[Job]) -> Result<Vec<(Uuid, f64)>, RecruitmentError> {
        let mut matches = Vec::new();
        
        for job in jobs {
            let score = self.calculate_job_candidate_match(job, candidate)?;
            if score > 80.0 { // Only include strong matches
                matches.push((job.id, score));
            }
        }
        
        // Sort by score descending
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        Ok(matches)
    }
}