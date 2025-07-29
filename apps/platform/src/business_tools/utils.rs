use std::collections::HashMap;

pub fn format_currency(amount: f64) -> String {
    format!("${:,.2}", amount)
}

pub fn format_percentage(value: f64) -> String {
    format!("{:.1}%", value)
}

pub fn calculate_growth_rate(current: f64, previous: f64) -> f64 {
    if previous == 0.0 {
        0.0
    } else {
        ((current - previous) / previous) * 100.0
    }
}

pub fn generate_sample_data(months: usize) -> Vec<(String, f64)> {
    let mut data = Vec::new();
    let base_revenue = 10000.0;
    
    for i in 0..months {
        let month_name = match i % 12 {
            0 => "January",
            1 => "February",
            2 => "March",
            3 => "April",
            4 => "May",
            5 => "June",
            6 => "July",
            7 => "August",
            8 => "September",
            9 => "October",
            10 => "November",
            11 => "December",
            _ => "Unknown",
        };
        
        let growth_factor = 1.0 + (i as f64 * 0.05);
        let revenue = base_revenue * growth_factor * (0.9 + rand::random::<f64>() * 0.2);
        
        data.push((month_name.to_string(), revenue));
    }
    
    data
}

pub fn categorize_impact_score(score: f64) -> &'static str {
    match score {
        s if s >= 9.0 => "Excellent",
        s if s >= 7.5 => "Good",
        s if s >= 6.0 => "Fair",
        _ => "Needs Improvement",
    }
}

pub fn get_impact_color(score: f64) -> &'static str {
    match score {
        s if s >= 9.0 => "#10b981",
        s if s >= 7.5 => "#3b82f6",
        s if s >= 6.0 => "#f59e0b",
        _ => "#ef4444",
    }
}

pub fn calculate_weighted_average(values: &[(f64, f64)]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let total_weight: f64 = values.iter().map(|(_, weight)| weight).sum();
    if total_weight == 0.0 {
        return 0.0;
    }
    
    let weighted_sum: f64 = values.iter().map(|(value, weight)| value * weight).sum();
    weighted_sum / total_weight
}

pub fn validate_impact_weights(weights: &HashMap<String, f64>) -> Result<(), String> {
    let total: f64 = weights.values().sum();
    
    if (total - 1.0).abs() > 0.001 {
        return Err(format!("Weights must sum to 1.0, current sum: {:.3}", total));
    }
    
    for (category, weight) in weights {
        if *weight < 0.0 || *weight > 1.0 {
            return Err(format!("Weight for {} must be between 0 and 1", category));
        }
    }
    
    Ok(())
}

pub fn format_date(date_str: &str) -> String {
    // Simple date formatting - in real app, use proper date parsing
    date_str.to_string()
}

pub fn generate_report_summary(data: &HashMap<String, f64>) -> String {
    let total = data.values().sum::<f64>();
    let average = total / data.len() as f64;
    
    format!(
        "Report Summary: Total impact across {} categories is ${:.2} with an average of ${:.2} per category.",
        data.len(),
        total,
        average
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(1234.56), "$1,234.56");
        assert_eq!(format_currency(0.0), "$0.00");
    }

    #[test]
    fn test_calculate_growth_rate() {
        assert_eq!(calculate_growth_rate(110.0, 100.0), 10.0);
        assert_eq!(calculate_growth_rate(100.0, 100.0), 0.0);
        assert_eq!(calculate_growth_rate(100.0, 0.0), 0.0);
    }

    #[test]
    fn test_categorize_impact_score() {
        assert_eq!(categorize_impact_score(9.5), "Excellent");
        assert_eq!(categorize_impact_score(8.0), "Good");
        assert_eq!(categorize_impact_score(6.5), "Fair");
        assert_eq!(categorize_impact_score(5.0), "Needs Improvement");
    }

    #[test]
    fn test_validate_impact_weights() {
        let mut weights = HashMap::new();
        weights.insert("Environment".to_string(), 0.4);
        weights.insert("Community".to_string(), 0.3);
        weights.insert("Workers".to_string(), 0.3);
        
        assert!(validate_impact_weights(&weights).is_ok());
        
        weights.insert("Environment".to_string(), 0.5);
        assert!(validate_impact_weights(&weights).is_err());
    }
}
