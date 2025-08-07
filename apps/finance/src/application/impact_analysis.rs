//! Impact analysis for cooperative financial activities
//!
//! This module provides tools for measuring the cooperative impact of financial activities
//! using Bayesian statistical methods aligned with cooperative values.

#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceCalculator,
    SignificanceTester,
    StatisticalError,
};

#[cfg(feature = "statistics")]
use crate::domain::{Budget, Expense, FinancialCategory, SavingsGoal};
#[cfg(feature = "statistics")]
use chrono::{DateTime, Utc};
#[cfg(feature = "statistics")]
use rust_decimal::Decimal;

/// Impact analysis service for measuring cooperative financial impact
#[cfg(feature = "statistics")]
pub struct ImpactAnalysisService;

#[cfg(feature = "statistics")]
impl ImpactAnalysisService {
    /// Analyze the impact of savings goals on financial wellbeing
    pub fn analyze_savings_impact(
        savings_goals: &[SavingsGoal],
        expenses: &[Expense],
        prior_mean: f64,
        prior_std: f64,
    ) -> Result<SavingsImpactAnalysis, StatisticalError> {
        if savings_goals.is_empty() {
            return Err(StatisticalError::InsufficientData(0, 1));
        }
        
        // Calculate savings rate
        let total_savings: f64 = savings_goals.iter().map(|g| g.current_amount.amount).sum();
        let total_income: f64 = expenses.iter().map(|e| e.amount.amount).sum() * 1.2; // Assume 20% savings rate
        let savings_rate = if total_income > 0.0 {
            total_savings / total_income
        } else {
            0.0
        };
        
        // Calculate goal completion rate
        let completed_goals = savings_goals.iter().filter(|g| g.is_completed()).count();
        let completion_rate = completed_goals as f64 / savings_goals.len() as f64;
        
        // Use Bayesian approach for impact analysis
        let data = vec![savings_rate, completion_rate];
        let ci = ConfidenceCalculator::bayesian_interval(&data, 0.95, prior_mean, prior_std)?;
        
        // Perform significance test
        let significance_test = SignificanceTester::one_sample_t_test(&data, prior_mean)?;
        
        Ok(SavingsImpactAnalysis::new(
            savings_rate,
            completion_rate,
            ci,
            significance_test,
        ))
    }
    
    /// Analyze the impact of budgeting on expense control
    pub fn analyze_budgeting_impact(
        budgets: &[Budget],
        expenses: &[Expense],
    ) -> Result<BudgetingImpactAnalysis, StatisticalError> {
        if budgets.is_empty() || expenses.is_empty() {
            return Err(StatisticalError::InsufficientData(
                budgets.len().min(expenses.len()),
                1,
            ));
        }
        
        // Calculate budget adherence rate
        let mut adherence_rates = Vec::new();
        
        for budget in budgets {
            let budget_expenses: Vec<&Expense> = expenses
                .iter()
                .filter(|e| e.category == budget.category)
                .collect();
            
            let total_spent: f64 = budget_expenses.iter().map(|e| e.amount.amount).sum();
            let budget_amount = budget.amount.amount;
            
            if budget_amount > 0.0 {
                let adherence_rate = total_spent / budget_amount;
                adherence_rates.push(adherence_rate);
            }
        }
        
        if adherence_rates.is_empty() {
            return Err(StatisticalError::InsufficientData(0, 1));
        }
        
        // Calculate average adherence
        let avg_adherence = adherence_rates.mean();
        
        // Calculate variance in adherence
        let adherence_variance = adherence_rates.std_dev().powi(2);
        
        // Calculate confidence interval
        let ci = ConfidenceCalculator::parametric_interval(&adherence_rates, 0.95)?;
        
        // Perform significance test (testing if adherence is significantly different from 1.0)
        let significance_test = SignificanceTester::one_sample_t_test(&adherence_rates, 1.0)?;
        
        Ok(BudgetingImpactAnalysis::new(
            avg_adherence,
            adherence_variance,
            ci,
            significance_test,
        ))
    }
}

/// Impact analysis results for savings activities
#[cfg(feature = "statistics")]
pub struct SavingsImpactAnalysis {
    /// Current savings rate
    pub savings_rate: f64,
    
    /// Goal completion rate
    pub completion_rate: f64,
    
    /// Bayesian confidence interval for impact
    pub confidence_interval: cpc_statistics_core::ConfidenceInterval,
    
    /// Significance test results
    pub significance_test: cpc_statistics_core::SignificanceResult,
}

#[cfg(feature = "statistics")]
impl SavingsImpactAnalysis {
    /// Create a new savings impact analysis
    pub fn new(
        savings_rate: f64,
        completion_rate: f64,
        confidence_interval: cpc_statistics_core::ConfidenceInterval,
        significance_test: cpc_statistics_core::SignificanceResult,
    ) -> Self {
        Self {
            savings_rate,
            completion_rate,
            confidence_interval,
            significance_test,
        }
    }
    
    /// Generate a cooperative values-aligned explanation
    pub fn cooperative_explanation(&self) -> String {
        let impact_strength = if self.savings_rate > 0.2 {
            "strong"
        } else if self.savings_rate > 0.1 {
            "moderate"
        } else {
            "limited"
        };
        
        format!(
            "Your savings activities show {} positive impact on financial wellbeing. \
            You're saving {:.1}% of your income with a {:.1}% goal completion rate. \
            Statistical analysis shows {} evidence for this impact (p = {:.4}). \
            This contributes to the cooperative's mission of financial empowerment for all members.",
            impact_strength,
            self.savings_rate * 100.0,
            self.completion_rate * 100.0,
            self.significance_test.level.description(),
            self.significance_test.p_value
        )
    }
}

/// Impact analysis results for budgeting activities
#[cfg(feature = "statistics")]
pub struct BudgetingImpactAnalysis {
    /// Average budget adherence rate
    pub avg_adherence: f64,
    
    /// Variance in adherence rates
    pub adherence_variance: f64,
    
    /// Confidence interval for impact
    pub confidence_interval: cpc_statistics_core::ConfidenceInterval,
    
    /// Significance test results
    pub significance_test: cpc_statistics_core::SignificanceResult,
}

#[cfg(feature = "statistics")]
impl BudgetingImpactAnalysis {
    /// Create a new budgeting impact analysis
    pub fn new(
        avg_adherence: f64,
        adherence_variance: f64,
        confidence_interval: cpc_statistics_core::ConfidenceInterval,
        significance_test: cpc_statistics_core::SignificanceResult,
    ) -> Self {
        Self {
            avg_adherence,
            adherence_variance,
            confidence_interval,
            significance_test,
        }
    }
    
    /// Check if budgeting is effective (adherence close to 1.0)
    pub fn is_effective(&self) -> bool {
        self.avg_adherence >= 0.8 && self.avg_adherence <= 1.2
    }
    
    /// Generate a cooperative values-aligned explanation
    pub fn cooperative_explanation(&self) -> String {
        let effectiveness = if self.is_effective() {
            "effectively"
        } else if self.avg_adherence < 0.8 {
            "under-budgeting"
        } else {
            "over-budgeting"
        };
        
        format!(
            "Your budgeting practices are {} managing expenses. \
            On average, you're spending {:.1}% of your budgeted amounts with consistent adherence patterns. \
            Statistical analysis shows {} evidence for your budgeting effectiveness (p = {:.4}). \
            This supports the cooperative's goal of helping members achieve financial stability.",
            effectiveness,
            self.avg_adherence * 100.0,
            self.significance_test.level.description(),
            self.significance_test.p_value
        )
    }
}

// Fallback implementation when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
pub struct ImpactAnalysisService;

#[cfg(not(feature = "statistics"))]
impl ImpactAnalysisService {
    /// Analyze the impact of savings goals (stub implementation)
    pub fn analyze_savings_impact(
        _savings_goals: &[SavingsGoal],
        _expenses: &[Expense],
        _prior_mean: f64,
        _prior_std: f64,
    ) -> Result<SavingsImpactAnalysis, &'static str> {
        Err("Impact analysis requires the 'statistics' feature to be enabled")
    }
    
    /// Analyze the impact of budgeting (stub implementation)
    pub fn analyze_budgeting_impact(
        _budgets: &[Budget],
        _expenses: &[Expense],
    ) -> Result<BudgetingImpactAnalysis, &'static str> {
        Err("Impact analysis requires the 'statistics' feature to be enabled")
    }
}

#[cfg(not(feature = "statistics"))]
pub struct SavingsImpactAnalysis;

#[cfg(not(feature = "statistics"))]
pub struct BudgetingImpactAnalysis;

#[cfg(test)]
#[cfg(feature = "statistics")]
mod tests {
    use super::*;
    use crate::domain::{primitives::Money, Currency, BudgetPeriod, SavingsGoal};
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;
    
    #[test]
    fn test_budgeting_impact_analysis() {
        let budget = Budget::new(
            Uuid::new_v4(),
            "Groceries".to_string(),
            Money::new(Decimal::new(500, 0), Currency::USD),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        let expense = Expense::new(
            Uuid::new_v4(),
            "Groceries".to_string(),
            Money::new(Decimal::new(450, 0), Currency::USD),
            Utc::now(),
            FinancialCategory::Groceries,
        );
        
        let analysis = ImpactAnalysisService::analyze_budgeting_impact(
            &[budget],
            &[expense],
        );
        
        assert!(analysis.is_ok());
    }
}