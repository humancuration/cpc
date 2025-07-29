use std::collections::HashMap;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BusinessData {
    pub total_revenue: f64,
    pub revenue_growth: f64,
    pub impact_score: f64,
    pub impact_improvement: f64,
    pub active_projects: usize,
    pub community_investment: f64,
    pub investment_growth: f64,
    pub monthly_metrics: HashMap<String, f64>,
}

impl Default for BusinessData {
    fn default() -> Self {
        Self {
            total_revenue: 125_000.0,
            revenue_growth: 15.3,
            impact_score: 8.7,
            impact_improvement: 0.5,
            active_projects: 12,
            community_investment: 45_000.0,
            investment_growth: 22.1,
            monthly_metrics: HashMap::from([
                ("January".to_string(), 10_000.0),
                ("February".to_string(), 12_500.0),
                ("March".to_string(), 15_000.0),
                ("April".to_string(), 13_200.0),
                ("May".to_string(), 16_800.0),
                ("June".to_string(), 18_500.0),
            ]),
        }
    }
}

#[hook]
pub fn use_business_data() -> BusinessData {
    use_state(BusinessData::default).get_cloned()
}

#[hook]
pub fn use_impact_categories() -> Vec<String> {
    use_memo(
        |_| vec![
            "Environment".to_string(),
            "Community".to_string(),
            "Workers".to_string(),
            "Governance".to_string(),
            "Supply Chain".to_string(),
        ],
        (),
    )
    .to_vec()
}

#[hook]
pub fn use_time_periods() -> Vec<String> {
    use_memo(
        |_| vec![
            "Last 7 days".to_string(),
            "Last 30 days".to_string(),
            "Last 90 days".to_string(),
            "Last year".to_string(),
            "All time".to_string(),
        ],
        (),
    )
    .to_vec()
}

#[derive(Debug, Clone, PartialEq)]
pub struct FilterState {
    pub category: Option<String>,
    pub time_period: String,
    pub organization_id: Option<String>,
}

impl Default for FilterState {
    fn default() -> Self {
        Self {
            category: None,
            time_period: "Last 30 days".to_string(),
            organization_id: None,
        }
    }
}

#[hook]
pub fn use_filters() -> (FilterState, Callback<FilterState>) {
    let filters = use_state(FilterState::default);
    
    let update_filters = {
        let filters = filters.clone();
        Callback::from(move |new_filters: FilterState| {
            filters.set(new_filters);
        })
    };
    
    (filters.get_cloned(), update_filters)
}
