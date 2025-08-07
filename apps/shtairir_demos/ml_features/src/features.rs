//! Feature engineering utilities for ML pipelines

use tracing::debug;

/// Represents feature engineering results
#[derive(Debug, Clone)]
pub struct FeatureEngineeringResult {
    pub original_features: usize,
    pub engineered_features: usize,
    pub interactions_added: usize,
}

/// Add polynomial features to a sample
pub fn add_polynomial_features(sample: &[f64], degree: usize) -> Vec<f64> {
    let mut extended = sample.to_vec();
    
    // Add squared terms for each feature
    for &feature in sample {
        extended.push(feature * feature);
    }
    
    // Add cubic terms if degree >= 3
    if degree >= 3 {
        for &feature in sample {
            extended.push(feature * feature * feature);
        }
    }
    
    extended
}

/// Add interaction features between features
pub fn add_interaction_features(sample: &[f64]) -> Vec<f64> {
    let mut extended = sample.to_vec();
    
    // Add interaction terms between consecutive features
    for i in 0..sample.len().saturating_sub(1) {
        extended.push(sample[i] * sample[i + 1]);
    }
    
    // Add interaction between first and last features
    if sample.len() > 2 {
        extended.push(sample[0] * sample[sample.len() - 1]);
    }
    
    extended
}

/// Normalize features using z-score normalization
pub fn normalize_features(sample: &[f64], means: &[f64], stds: &[f64]) -> Vec<f64> {
    sample.iter()
        .zip(means.iter())
        .zip(stds.iter())
        .map(|((x, mean), std)| {
            if *std != 0.0 {
                (x - mean) / std
            } else {
                *x - mean
            }
        })
        .collect()
}

/// Compute means for each feature column
pub fn compute_feature_means(dataset: &[Vec<f64>]) -> Vec<f64> {
    if dataset.is_empty() {
        return Vec::new();
    }
    
    let feature_count = dataset[0].len();
    let mut means = vec![0.0; feature_count];
    
    for i in 0..feature_count {
        let sum: f64 = dataset.iter().map(|sample| sample[i]).sum();
        means[i] = sum / dataset.len() as f64;
    }
    
    means
}

/// Compute standard deviations for each feature column
pub fn compute_feature_stds(dataset: &[Vec<f64>], means: &[f64]) -> Vec<f64> {
    if dataset.is_empty() || means.is_empty() {
        return Vec::new();
    }
    
    let feature_count = dataset[0].len();
    let mut stds = vec![0.0; feature_count];
    
    for i in 0..feature_count {
        let variance: f64 = dataset.iter()
            .map(|sample| (sample[i] - means[i]).powi(2))
            .sum::<f64>() / dataset.len() as f64;
        stds[i] = variance.sqrt();
    }
    
    stds
}

/// Create train/test split indices
pub fn create_train_test_split(dataset_size: usize, test_ratio: f64) -> (Vec<usize>, Vec<usize>) {
    use rand::seq::SliceRandom;
    
    let mut indices: Vec<usize> = (0..dataset_size).collect();
    let mut rng = rand::thread_rng();
    indices.shuffle(&mut rng);
    
    let test_size = (dataset_size as f64 * test_ratio) as usize;
    let (test_indices, train_indices) = indices.split_at(test_size);
    
    (train_indices.to_vec(), test_indices.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_polynomial_features() {
        let sample = vec![1.0, 2.0, 3.0];
        let extended = add_polynomial_features(&sample, 2);
        
        // Original features + squared terms
        assert_eq!(extended.len(), 6);
        assert_eq!(extended[3], 1.0); // 1^2
        assert_eq!(extended[4], 4.0); // 2^2
        assert_eq!(extended[5], 9.0); // 3^2
    }
    
    #[test]
    fn test_interaction_features() {
        let sample = vec![1.0, 2.0, 3.0];
        let extended = add_interaction_features(&sample);
        
        // Original features + interaction terms
        assert_eq!(extended.len(), 5);
        assert_eq!(extended[3], 2.0); // 1*2
        assert_eq!(extended[4], 3.0); // 2*3
    }
    
    #[test]
    fn test_normalize_features() {
        let sample = vec![1.0, 2.0, 3.0];
        let means = vec![1.0, 2.0, 3.0];
        let stds = vec![0.5, 1.0, 1.5];
        
        let normalized = normalize_features(&sample, &means, &stds);
        assert_eq!(normalized, vec![0.0, 0.0, 0.0]);
    }
    
    #[test]
    fn test_compute_means() {
        let dataset = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 3.0, 4.0],
            vec![3.0, 4.0, 5.0],
        ];
        
        let means = compute_feature_means(&dataset);
        assert_eq!(means, vec![2.0, 3.0, 4.0]);
    }
}