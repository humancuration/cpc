//! Accessibility utilities for generating alt text and ensuring WCAG compliance

use reviews::Review;
use crate::data_generator::generators::products::Product;
use web_sys::HtmlCanvasElement;

/// Generate alt text for summary visualization
pub fn generate_summary_alt_text(reviews: &[Review<Product>]) -> String {
    if reviews.is_empty() {
        return "Feedback summary visualization showing no reviews".to_string();
    }
    
    let total_reviews = reviews.len();
    let avg_rating = calculate_average_rating(reviews);
    
    format!(
        "Feedback summary showing {} reviews with an average rating of {:.1} out of 5 stars",
        total_reviews,
        avg_rating * 5.0
    )
}

/// Generate alt text for ratings chart visualization
pub fn generate_ratings_chart_alt_text(reviews: &[Review<Product>]) -> String {
    if reviews.is_empty() {
        return "Ratings distribution chart showing no data".to_string();
    }
    
    let distribution = calculate_rating_distribution(reviews);
    
    format!(
        "Ratings distribution chart showing {} reviews. Distribution: {} 1-star, {} 2-star, {} 3-star, {} 4-star, {} 5-star ratings",
        reviews.len(),
        distribution[0], distribution[1], distribution[2], distribution[3], distribution[4]
    )
}

/// Generate alt text for word cloud visualization
pub fn generate_word_cloud_alt_text(reviews: &[Review<Product>]) -> String {
    if reviews.is_empty() {
        return "Word cloud visualization showing no data".to_string();
    }
    
    let common_words = get_common_words(reviews, 5);
    
    if common_words.is_empty() {
        return "Word cloud visualization showing no significant words".to_string();
    }
    
    let word_list = common_words
        .iter()
        .map(|(word, count)| format!("{} ({})", word, count))
        .collect::<Vec<_>>()
        .join(", ");
    
    format!(
        "Word cloud visualization showing common words in reviews. Most frequent: {}",
        word_list
    )
}

/// Generate alt text for sentiment analysis visualization
pub fn generate_sentiment_alt_text(reviews: &[Review<Product>]) -> String {
    if reviews.is_empty() {
        return "Sentiment analysis visualization showing no data".to_string();
    }
    
    let (positive, neutral, negative) = calculate_sentiment_distribution(reviews);
    let total = positive + neutral + negative;
    
    format!(
        "Sentiment analysis visualization showing {} reviews. Positive: {} ({}%), Neutral: {} ({}%), Negative: {} ({}%)",
        total,
        positive,
        percentage(positive, total),
        neutral,
        percentage(neutral, total),
        negative,
        percentage(negative, total)
    )
}

/// Calculate average rating from reviews
fn calculate_average_rating(reviews: &[Review<Product>]) -> f32 {
    if reviews.is_empty() {
        return 0.0;
    }
    
    let sum: f32 = reviews
        .iter()
        .filter_map(|review| {
            review
                .ratings
                .iter()
                .find(|rating| rating.metric == "overall")
                .map(|rating| rating.value)
        })
        .sum();
    
    sum / reviews.len() as f32
}

/// Calculate rating distribution (1-5 stars)
fn calculate_rating_distribution(reviews: &[Review<Product>]) -> [usize; 5] {
    let mut distribution = [0; 5];
    
    for review in reviews {
        if let Some(overall_rating) = review
            .ratings
            .iter()
            .find(|rating| rating.metric == "overall")
        {
            let star_rating = (overall_rating.value * 5.0).round() as usize;
            let index = star_rating.saturating_sub(1).min(4);
            distribution[index] += 1;
        }
    }
    
    distribution
}

/// Calculate sentiment distribution
fn calculate_sentiment_distribution(reviews: &[Review<Product>]) -> (usize, usize, usize) {
    let mut positive = 0;
    let mut neutral = 0;
    let mut negative = 0;
    
    for review in reviews {
        // Simple sentiment analysis based on overall rating
        if let Some(overall_rating) = review
            .ratings
            .iter()
            .find(|rating| rating.metric == "overall")
        {
            if overall_rating.value > 0.7 {
                positive += 1;
            } else if overall_rating.value < 0.4 {
                negative += 1;
            } else {
                neutral += 1;
            }
        }
    }
    
    (positive, neutral, negative)
}

/// Get most common words from reviews
fn get_common_words(reviews: &[Review<Product>], count: usize) -> Vec<(String, usize)> {
    use std::collections::HashMap;
    
    let mut word_counts = HashMap::new();
    
    // Common stop words to filter out
    let stop_words = [
        "the", "and", "for", "are", "but", "not", "you", "all", "can", "had", "her", "was", 
        "one", "our", "out", "day", "get", "has", "him", "his", "how", "its", "may", "new", 
        "now", "old", "see", "two", "who", "boy", "did", "man", "men", "put", "too", "use", 
        "any", "big", "end", "far", "got", "hot", "let", "lot", "run", "set", "sit", "way",
        "will", "with", "this", "that", "have", "from", "were", "been", "have", "they", "them",
        "than", "then", "what", "when", "where", "which", "who", "why", "would", "could", "should",
        "might", "must", "shall", "does", "done", "very", "just", "only", "even", "also", "well",
        "here", "there", "about", "into", "over", "after", "before", "under", "above", "below"
    ];
    
    for review in reviews {
        for word in review.content.split_whitespace() {
            // Clean word (remove punctuation, convert to lowercase)
            let clean_word = word
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>()
                .to_lowercase();
            
            // Skip empty words and stop words
            if clean_word.is_empty() || clean_word.len() < 3 || stop_words.contains(&clean_word.as_str()) {
                continue;
            }
            
            // Increment frequency count
            *word_counts.entry(clean_word).or_insert(0) += 1;
        }
    }
    
    // Sort by frequency and take top N
    let mut sorted_words: Vec<(String, usize)> = word_counts.into_iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_words.truncate(count);
    
    sorted_words
}

/// Calculate percentage
fn percentage(part: usize, total: usize) -> usize {
    if total == 0 {
        0
    } else {
        (part as f32 / total as f32 * 100.0).round() as usize
    }
}

/// Check color contrast ratio (WCAG compliance)
pub fn check_color_contrast(background: (u8, u8, u8), foreground: (u8, u8, u8)) -> f32 {
    let bg_luminance = calculate_luminance(background);
    let fg_luminance = calculate_luminance(foreground);
    
    let lighter = bg_luminance.max(fg_luminance);
    let darker = bg_luminance.min(fg_luminance);
    
    (lighter + 0.05) / (darker + 0.05)
}

/// Calculate relative luminance of a color
fn calculate_luminance(rgb: (u8, u8, u8)) -> f32 {
    let r = rgb.0 as f32 / 255.0;
    let g = rgb.1 as f32 / 255.0;
    let b = rgb.2 as f32 / 255.0;
    
    let r = if r <= 0.03928 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
    let g = if g <= 0.03928 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
    let b = if b <= 0.03928 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };
    
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Generate alt text for shared visualization
pub fn generate_sharing_alt_text(
    visualization_type: &str,
    reviews: &[Review<Product>]
) -> String {
    format!("Shared {} visualization showing {} reviews", visualization_type, reviews.len())
}

/// Ensure social button accessibility attributes
pub fn ensure_social_button_accessibility() -> Vec<(&'static str, &'static str)> {
    vec![
        ("role", "button"),
        ("aria-label", "Sharing options"),
        ("tabindex", "0")
    ]
}