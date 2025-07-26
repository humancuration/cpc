use async_graphql::*;
use crate::FeatureFlags;

pub struct FeatureFlagsQuery;

#[Object]
impl FeatureFlagsQuery {
    /// Get current feature flags configuration
    async fn feature_flags(&self, ctx: &Context<'_>) -> FeatureFlags {
        ctx.data_unchecked::<FeatureFlags>().clone()
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct FeatureFlags {
    pub ui_degradation_threshold: f64,
}

impl FeatureFlags {
    pub fn new(config: &crate::config::Config) -> Self {
        Self {
            ui_degradation_threshold: config.ui_thresholds.degradation,
        }
    }
}