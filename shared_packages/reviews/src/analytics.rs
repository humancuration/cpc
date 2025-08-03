//! Analytics engine for reviews
//!
//! This module provides statistical analysis and visualization capabilities
//! for review data, including rating distributions, sentiment analysis, and trends.
// No  sentiment analysis. Only self reported ratings and advanced statistics based on that data.

use std::collections::HashMap;
use plotters::prelude::*;
use crate::models::{Review, Rating, Entity};
use crate::filters::ReviewFilters;
use feedback_analysis::{RatingDistribution, TrendResult};

/// Result type for plot operations
pub type PlotResult = Result<Vec<u8>, Box<dyn std::error::Error>>;

/// Analytics engine for processing review data
pub struct AnalyticsEngine;

impl AnalyticsEngine {
    /// Create a new analytics engine
    pub fn new() -> Self {
        Self
    }
    
    /// Calculate the average rating across filtered reviews
    pub fn average_rating<T: Entity>(&self, reviews: &[Review<T>], metric: &str) -> f32 {
        let mut sum = 0.0;
        let mut count = 0;
        
        for review in reviews {
            for rating in &review.ratings {
                if rating.metric == metric {
                    sum += rating.value;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            sum / count as f32
        } else {
            0.0
        }
    }
    /// Calculate the distribution of ratings for a specific metric
    pub fn rating_distribution<T: Entity>(
        &self,
        reviews: &[Review<T>],
        metric: &str
    ) -> RatingDistribution {
        let mut distribution = RatingDistribution::new(metric.to_string());
        
        for review in reviews {
            for rating in &review.ratings {
                if rating.metric == metric {
                    // Add rating to distribution (RatingDistribution handles scaling internally)
                    let _ = distribution.add_rating(rating.value);
                }
            }
        }
        
        distribution
    }
    }
    
    /// Perform basic sentiment analysis on review content
    ///
    /// This is a simplified sentiment analysis that looks for positive and negative words.
    /// In a real implementation, this would use a more sophisticated NLP approach.
    pub fn sentiment_analysis(&self, text: &str) -> f32 {
        let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "fantastic"];
        let negative_words = ["bad", "terrible", "awful", "horrible", "disappointing", "poor"];
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut positive_count = 0;
        let mut negative_count = 0;
        
        for word in words {
            let word_lower = word.to_lowercase();
            if positive_words.contains(&word_lower.as_str()) {
                positive_count += 1;
            } else if negative_words.contains(&word_lower.as_str()) {
                negative_count += 1;
            }
        }
        
        if positive_count + negative_count == 0 {
            0.0 // Neutral
        } else {
            (positive_count as f32 - negative_count as f32) / (positive_count + negative_count) as f32
        }
    }
    
    /// Generate a histogram of rating distribution
    pub fn plot_rating_distribution<T: Entity>(
        &self,
        reviews: &[Review<T>],
        metric: &str,
    ) -> PlotResult {
        let distribution = self.rating_distribution(reviews, metric);
        
        // Create an in-memory image buffer
        let mut buffer = vec![0; 800 * 600 * 3];
        
        {
            let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
            root.fill(&WHITE)?;
            
            let sorted_values = distribution.sorted_values();
            if sorted_values.is_empty() {
                root.present()?;
                return Ok(buffer);
            }
            
            let max_count = sorted_values.iter().map(|&(_, count)| count).max().unwrap_or(1);
            
            let mut chart = ChartBuilder::on(&root)
                .caption(format!("Rating Distribution for {}", metric), ("sans-serif", 30))
                .margin(10)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0u8..=100u8, 0u32..=max_count)?;
            
            chart.configure_mesh().draw()?;
            
            // Draw bars for each rating value
            chart.draw_series(
                sorted_values
                    .iter()
                    .map(|&(value, count)| {
                        let bar = Rectangle::new(
                            [(value.saturating_sub(2), 0), (value + 2, count)],
                            BLUE.filled(),
                        );
                        bar
                    /// Generate a line chart showing rating trends over time
                    pub fn plot_rating_trends<T: Entity>(
                        &self,
                        reviews: &[Review<T>],
                        metric: &str,
                    ) -> PlotResult {
                        // Group ratings by date (simplified to day-level for this example)
                        let mut ratings_by_day: HashMap<chrono::NaiveDate, Vec<f32>> = HashMap::new();
                        
                        for review in reviews {
                            for rating in &review.ratings {
                                if rating.metric == metric {
                                    let date = review.created_at.date_naive();
                                    ratings_by_day.entry(date).or_insert_with(Vec::new).push(rating.value);
                                }
                            }
                        }
                        
                        // Calculate average rating per day
                        let mut trend_result = TrendResult::new();
                        let mut data_points: Vec<(chrono::NaiveDate, f32, usize)> = ratings_by_day
                            .iter()
                            .map(|(date, ratings)| {
                                let avg = ratings.iter().sum::<f32>() / ratings.len() as f32;
                                let count = ratings.len();
                                (*date, avg, count)
                            })
                            .collect();
                        
                        // Sort by date
                        data_points.sort_by_key(|(date, _, _)| *date);
                        
                        // Populate trend result
                        for (date, avg, count) in data_points {
                            trend_result.add_point(date.to_string(), avg, count);
                        }
                        
                        // Create an in-memory image buffer
                        let mut buffer = vec![0; 800 * 600 * 3];
                        
                        {
                            let root = BitMapBackend::with_buffer(&mut buffer, (800, 600)).into_drawing_area();
                            root.fill(&WHITE)?;
                            
                            if !trend_result.is_empty() {
                                let min_rating = trend_result.averages.iter().fold(1.0f32, |a, &b| a.min(b));
                                let max_rating = trend_result.averages.iter().fold(0.0f32, |a, &b| a.max(b));
                                
                                // For simplicity, we'll use indices as x-axis values
                                let max_index = trend_result.len();
                                
                                let mut chart = ChartBuilder::on(&root)
                                    .caption(format!("Rating Trends for {}", metric), ("sans-serif", 30))
                                    .margin(10)
                                    .x_label_area_size(40)
                                    .y_label_area_size(40)
                                    .build_cartesian_2d(0..max_index, min_rating..max_rating)?;
                                
                                chart.configure_mesh().draw()?;
                                
                                let data: Vec<(usize, f32)> = trend_result.averages.iter().enumerate().map(|(i, &v)| (i, v)).collect();
                                chart.draw_series(LineSeries::new(data, &BLUE))?;
                            }
                            
                            root.present()?;
                        }
                        
                        Ok(buffer)
                    }
                    &BLUE,
                ))?;
            }
            
            root.present()?;
        }
        
        Ok(buffer)
    }
    
    /// Compare entities based on their average ratings
    pub fn compare_entities<T: Entity>(
        &self,
        reviews: &[Review<T>],
        entity_ids: &[uuid::Uuid],
        metric: &str,
    ) -> HashMap<uuid::Uuid, f32> {
        let mut entity_ratings: HashMap<uuid::Uuid, Vec<f32>> = HashMap::new();
        
        // Group ratings by entity
        for review in reviews {
            if entity_ids.contains(&review.entity.id()) {
                for rating in &review.ratings {
                    if rating.metric == metric {
                        entity_ratings
                            .entry(review.entity.id())
                            .or_insert_with(Vec::new)
                            .push(rating.value);
                    }
                }
            }
        }
        
        // Calculate average rating per entity
        entity_ratings
            .into_iter()
            .map(|(entity_id, ratings)| {
                let avg = ratings.iter().sum::<f32>() / ratings.len() as f32;
                (entity_id, avg)
            })
            .collect()
    }
}

impl Default for AnalyticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{TestEntity, RatingMethod};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_average_rating() {
        let entity = TestEntity {
            id: Uuid::new_v4(),
            name: "Test Product".to_string(),
        };
        
        let reviews = vec![
            Review {
                id: Uuid::new_v4(),
                entity: entity.clone(),
                user_id: Uuid::new_v4(),
                title: "Great product".to_string(),
                content: "I love this product".to_string(),
                ratings: vec![
                    Rating {
                        metric: "quality".to_string(),
                        value: 0.9,
                        unit: None,
                        method: RatingMethod::UserReported,
                    }
                ],
                attributes: vec![],
                demographics: None,
                created_at: Utc::now(),
            },
            Review {
                id: Uuid::new_v4(),
                entity: entity.clone(),
                user_id: Uuid::new_v4(),
                title: "Good product".to_string(),
                content: "This is good".to_string(),
                ratings: vec![
                    Rating {
                        metric: "quality".to_string(),
                        value: 0.7,
                        unit: None,
                        method: RatingMethod::UserReported,
                    }
                ],
                attributes: vec![],
                demographics: None,
                created_at: Utc::now(),
            }
        ];
        
        let engine = AnalyticsEngine::new();
        let avg = engine.average_rating(&reviews, "quality");
        
        assert_eq!(avg, 0.8); // (0.9 + 0.7) / 2
    }
    
    #[test]
    fn test_sentiment_analysis() {
        let engine = AnalyticsEngine::new();
        
        let positive_text = "This product is great and amazing";
        let negative_text = "This product is terrible and awful";
        let neutral_text = "This product is okay";
        
        let positive_sentiment = engine.sentiment_analysis(positive_text);
        let negative_sentiment = engine.sentiment_analysis(negative_text);
        let neutral_sentiment = engine.sentiment_analysis(neutral_text);
        
        assert!(positive_sentiment > 0.0);
        assert!(negative_sentiment < 0.0);
        assert_eq!(neutral_sentiment, 0.0);
    }
}