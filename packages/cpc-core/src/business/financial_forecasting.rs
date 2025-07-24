//! Financial forecasting module for cash flow projections and scenario modeling
use chrono::{Date, Utc, Duration, Months, TimeDelta};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use crate::business::accounting::{Account, Transaction, FinancialStatement};
use rand::prelude::*;
use rand_distr::Normal;

/// Financial forecasting errors
#[derive(Debug, Error)]
pub enum ForecastError {
    #[error("Scenario not found")]
    ScenarioNotFound,
    #[error("Insufficient historical data: {0} transactions available")]
    InsufficientData(usize),
    #[error("Invalid date range: {0} to {1}")]
    InvalidDateRange(Date<Utc>, Date<Utc>),
    #[error("Data validation failed: {0}")]
    ValidationFailed(String),
    #[error("Algorithm not supported: {0}")]
    UnsupportedAlgorithm(String),
    #[error("Permission denied for resource: {0}")]
    PermissionDenied(String),
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    #[error("External service error: {0}")]
    ExternalServiceError(String),
}

/// Parameters for financial forecasting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastParameters {
    /// Forecast start date
    pub start_date: Date<Utc>,
    /// Forecast end date
    pub end_date: Date<Utc>,
    /// Time interval for projections (e.g., "monthly", "quarterly")
    pub interval: String,
    /// Scenario parameters (key-value pairs)
    pub scenario_parameters: HashMap<String, f64>,
}

/// Cash flow projection for a given period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowProjection {
    /// Projection date
    pub date: Date<Utc>,
    /// Projected cash inflow
    pub inflow: f64,
    /// Projected cash outflow
    pub outflow: f64,
    /// Projected net cash flow
    pub net_cash_flow: f64,
}

/// Financial forecast scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Scenario name
    pub name: String,
    /// Scenario parameters
    pub parameters: ForecastParameters,
    /// Cash flow projections
    pub projections: Vec<CashFlowProjection>,
}

/// Financial forecast result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialForecast {
    /// Base forecast parameters
    pub base_parameters: ForecastParameters,
    /// Forecast scenarios
    pub scenarios: Vec<Scenario>,
}

impl FinancialForecast {
    /// Create a new financial forecast
    pub fn new(parameters: ForecastParameters) -> Self {
        FinancialForecast {
            base_parameters: parameters,
            scenarios: Vec::new(),
        }
    }

    /// Add a scenario to the forecast
    pub fn add_scenario(&mut self, name: String, parameters: ForecastParameters) {
        let scenario = Scenario {
            name,
            parameters,
            projections: Vec::new(),
        };
        self.scenarios.push(scenario);
    }

    /// Project cash flow using various forecasting algorithms
    pub fn project_cash_flow(&mut self, scenario_name: &str, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
        let scenario = self.scenarios.iter_mut()
            .find(|s| s.name == scenario_name)
            .ok_or(ForecastError::ScenarioNotFound)?;

        // Validate historical data
        if historical_transactions.is_empty() {
            return Err(ForecastError::InsufficientData(historical_transactions.len()));
        }

        // Apply selected algorithm based on scenario parameters
        let algorithm = scenario.parameters.scenario_parameters
            .get("algorithm")
            .map(|a| a.to_string())
            .unwrap_or_else(|| "exponential_smoothing".to_string());

        match algorithm.as_str() {
            "exponential_smoothing" => self.apply_exponential_smoothing(scenario, historical_transactions),
            "monte_carlo" => self.apply_monte_carlo(scenario, historical_transactions),
            "regression" => self.apply_regression_analysis(scenario, historical_transactions),
            "moving_average" => self.apply_moving_average(scenario, historical_transactions),
            _ => self.apply_exponential_smoothing(scenario, historical_transactions),
        }
    }

    /// Apply exponential smoothing algorithm
    fn apply_exponential_smoothing(&mut self, scenario: &mut Scenario, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
        // Group transactions by period
        let mut grouped = self.group_transactions(historical_transactions)?;
        
        // Apply exponential smoothing formula: F_t+1 = α * Y_t + (1-α) * F_t
        let alpha = scenario.parameters.scenario_parameters
            .get("alpha")
            .copied()
            .unwrap_or(0.3);

        // Calculate initial forecast as average of first 3 periods
        let periods: Vec<String> = grouped.keys().cloned().collect();
        let init_periods = periods.iter().take(3).collect::<Vec<_>>();
        let mut avg_inflow = init_periods.iter()
            .filter_map(|p| grouped.get(*p).map(|v| v.0))
            .sum::<f64>() / init_periods.len() as f64;
        
        let mut avg_outflow = init_periods.iter()
            .filter_map(|p| grouped.get(*p).map(|v| v.1))
            .sum::<f64>() / init_periods.len() as f64;

        // Apply smoothing to remaining periods
        for period in periods.iter().skip(3) {
            if let Some(&(inflow, outflow, _)) = grouped.get(period) {
                avg_inflow = alpha * inflow + (1.0 - alpha) * avg_inflow;
                avg_outflow = alpha * outflow + (1.0 - alpha) * avg_outflow;
            }
        }

        // Generate projections with growth factors
        self.generate_projections(scenario, avg_inflow, avg_outflow)
    }

    /// Group transactions by period and calculate totals
    fn group_transactions(&self, transactions: &[Transaction]) -> Result<HashMap<String, (f64, f64, usize)>, ForecastError> {
        let mut grouped = HashMap::new();
        for tx in transactions {
            let period = self.get_period_key(tx.date);
            let entry = grouped.entry(period).or_insert((0.0, 0.0, 0));
            if tx.amount > 0.0 {
                entry.0 += tx.amount;
            } else {
                entry.1 += tx.amount.abs();
            }
            entry.2 += 1;
        }
        
        if grouped.is_empty() {
            Err(ForecastError::InsufficientData(transactions.len()))
        } else {
            Ok(grouped)
        }
    }

    /// Generate projections based on calculated averages
    fn generate_projections(&mut self, scenario: &mut Scenario, avg_inflow: f64, avg_outflow: f64) -> Result<(), ForecastError> {
        let growth_factor = scenario.parameters.scenario_parameters
            .get("growth_rate")
            .copied()
            .unwrap_or(1.0);
        
        let cost_factor = scenario.parameters.scenario_parameters
            .get("cost_increase")
            .copied()
            .unwrap_or(1.0);

        let mut current_date = scenario.parameters.start_date;
        let mut projection_num = 0;
        let projection_count = (scenario.parameters.end_date - scenario.parameters.start_date).num_days() / 30;
        
        scenario.projections.clear();
        
        while current_date <= scenario.parameters.end_date {
            let growth_multiplier = 1.0 + (projection_num as f64 / projection_count as f64) * (growth_factor - 1.0);
            let cost_multiplier = 1.0 + (projection_num as f64 / projection_count as f64) * (cost_factor - 1.0);
            
            scenario.projections.push(CashFlowProjection {
                date: current_date,
                inflow: avg_inflow * growth_multiplier,
                outflow: avg_outflow * cost_multiplier,
                net_cash_flow: avg_inflow * growth_multiplier - avg_outflow * cost_multiplier,
            });
            
            projection_num += 1;
            match scenario.parameters.interval.as_str() {
                "monthly" => current_date = current_date + Months::new(1),
                "quarterly" => current_date = current_date + Months::new(3),
                _ => current_date = current_date + Duration::days(30),
            }
        }
        
        Ok(())
    }

    // Placeholder for other algorithms
    /// Apply Monte Carlo simulation for forecasting
    fn apply_monte_carlo(&mut self, scenario: &mut Scenario, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
        let grouped = self.group_transactions(historical_transactions)?;
        let inflows: Vec<f64> = grouped.values().map(|(i, _, _)| *i).collect();
        let outflows: Vec<f64> = grouped.values().map(|(_, o, _)| *o).collect();

        if inflows.len() < 2 || outflows.len() < 2 {
            return Err(ForecastError::InsufficientData(inflows.len()));
        }

        let inflow_mean = inflows.iter().sum::<f64>() / inflows.len() as f64;
        let inflow_std_dev = (inflows.iter().map(|x| (x - inflow_mean).powi(2)).sum::<f64>() / inflows.len() as f64).sqrt();

        let outflow_mean = outflows.iter().sum::<f64>() / outflows.len() as f64;
        let outflow_std_dev = (outflows.iter().map(|x| (x - outflow_mean).powi(2)).sum::<f64>() / outflows.len() as f64).sqrt();

        // For simplicity, we'll use a normal distribution. A more advanced model might use a different one.
        // This is a simplified simulation and not cryptographically secure.
        let mut rng = rand::thread_rng();
        let inflow_dist = rand_distr::Normal::new(inflow_mean, inflow_std_dev).unwrap();
        let outflow_dist = rand_distr::Normal::new(outflow_mean, outflow_std_dev).unwrap();

        let mut current_date = scenario.parameters.start_date;
        scenario.projections.clear();

        while current_date <= scenario.parameters.end_date {
            let simulated_inflow = rand_distr::Distribution::sample(&inflow_dist, &mut rng);
            let simulated_outflow = rand_distr::Distribution::sample(&outflow_dist, &mut rng);

            scenario.projections.push(CashFlowProjection {
                date: current_date,
                inflow: simulated_inflow.max(0.0), // Ensure non-negative
                outflow: simulated_outflow.max(0.0),
                net_cash_flow: simulated_inflow.max(0.0) - simulated_outflow.max(0.0),
            });

            match scenario.parameters.interval.as_str() {
                "monthly" => current_date = current_date + Months::new(1),
                "quarterly" => current_date = current_date + Months::new(3),
                _ => current_date = current_date + Duration::days(30),
            }
        }

        Ok(())
    }
fn apply_moving_average(&mut self, scenario: &mut Scenario, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
    // Calculate average monthly net cash flow
    let avg_monthly_net = historical_transactions
        .iter()
        .fold(0.0, |sum, t| sum + t.amount) / historical_transactions.len() as f64;
    
    // Extract scenario parameters
    let growth_factor = 1.0 + (scenario.parameters.scenario_parameters
        .get("growth_rate")
        .copied()
        .unwrap_or(0.0) / 100.0);
    
    let expense_factor = 1.0 + (scenario.parameters.scenario_parameters
        .get("cost_increase")
        .copied()
        .unwrap_or(0.0) / 100.0);
    
    // Generate projections
    let mut projections = Vec::new();
    let mut current_date = scenario.parameters.start_date;
    let projection_count = ((scenario.parameters.end_date - scenario.parameters.start_date).num_days() / 30) as usize;
    
    for month in 1..=projection_count {
        let projection = avg_monthly_net * growth_factor.powi(month as i32);
        projections.push(CashFlowProjection {
            date: current_date,
            inflow: projection,
            outflow: projection * expense_factor.powi(month as i32),
            net_cash_flow: projection - (projection * expense_factor.powi(month as i32)),
        });
        
        // Advance to next month
        current_date = current_date + chrono::Months::new(1);
    }
    
    scenario.projections = projections;
    Ok(())
}

    /// Apply linear regression for forecasting
    fn apply_regression_analysis(&mut self, scenario: &mut Scenario, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
        let grouped = self.group_transactions(historical_transactions)?;
        let mut periods: Vec<_> = grouped.keys().collect();
        periods.sort();

        let data_points: Vec<(f64, f64)> = periods.iter().enumerate().map(|(i, p)| (i as f64, grouped[*p].0 - grouped[*p].1)).collect();

        if data_points.len() < 2 {
            return Err(ForecastError::InsufficientData(data_points.len()));
        }

        // Simple linear regression: y = slope * x + intercept
        let n = data_points.len() as f64;
        let sum_x = data_points.iter().map(|(x, _)| x).sum::<f64>();
        let sum_y = data_points.iter().map(|(_, y)| y).sum::<f64>();
        let sum_xy = data_points.iter().map(|(x, y)| x * y).sum::<f64>();
        let sum_x2 = data_points.iter().map(|(x, _)| x.powi(2)).sum::<f64>();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;

        let mut current_date = scenario.parameters.start_date;
        scenario.projections.clear();
        let mut projection_num = data_points.len();

        while current_date <= scenario.parameters.end_date {
            let net_flow = slope * projection_num as f64 + intercept;
            // For simplicity, we'll assume a constant inflow/outflow ratio from the last historical period
            let last_period_key = periods.last().ok_or(ForecastError::InsufficientData(0))?;
            let (last_inflow, last_outflow, _) = grouped[*last_period_key];
            let ratio = if (last_inflow + last_outflow) > 0.0 { last_inflow / (last_inflow + last_outflow) } else { 0.5 };

            scenario.projections.push(CashFlowProjection {
                date: current_date,
                inflow: net_flow * ratio,
                outflow: net_flow * (1.0 - ratio),
                net_cash_flow: net_flow,
            });

            projection_num += 1;
            match scenario.parameters.interval.as_str() {
                "monthly" => current_date = current_date + Months::new(1),
                "quarterly" => current_date = current_date + Months::new(3),
                _ => current_date = current_date + Duration::days(30),
            }
        }

        Ok(())
    }
    }
    
    /// Calculate budget variance with detailed breakdown
    pub fn calculate_budget_variance(&self, actual_cash_flow: &[CashFlowProjection], scenario_name: &str) -> Result<HashMap<String, f64>, ForecastError> {
        let scenario = self.scenarios.iter()
            .find(|s| s.name == scenario_name)
            .ok_or(ForecastError::ScenarioNotFound)?;
        
        if scenario.projections.len() != actual_cash_flow.len() {
            return Err(ForecastError::InvalidDateRange);
        }
        
        let mut variances = HashMap::new();
        let mut total_variance = 0.0;
        let mut inflow_variance = 0.0;
        let mut outflow_variance = 0.0;
        
        for (i, (projected, actual)) in scenario.projections.iter().zip(actual_cash_flow.iter()).enumerate() {
            let period_variance = (projected.net_cash_flow - actual.net_cash_flow).abs();
            total_variance += period_variance;
            inflow_variance += (projected.inflow - actual.inflow).abs();
            outflow_variance += (projected.outflow - actual.outflow).abs();
            
            variances.insert(format!("Period {}", i+1), period_variance);
        }
        
        let count = actual_cash_flow.len() as f64;
        variances.insert("Average Variance".to_string(), total_variance / count);
        variances.insert("Total Inflow Variance".to_string(), inflow_variance);
        variances.insert("Total Outflow Variance".to_string(), outflow_variance);
        
        Ok(variances)
    }

    /// Get period key for grouping transactions
    fn get_period_key(&self, date: Date<Utc>) -> String {
        match self.base_parameters.interval.as_str() {
            "monthly" => format!("{}-{}", date.year(), date.month()),
            "quarterly" => {
                let quarter = (date.month() - 1) / 3 + 1;
                format!("{}-Q{}", date.year(), quarter)
            }
            _ => date.to_string(),
        }
    }
}

/// Parameters for sensitivity analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityParameters {
    /// Revenue growth percentage
    pub revenue_growth: f64,
    /// Expense change percentage
    pub expense_change: f64,
    /// Interest rate percentage
    pub interest_rate: f64,
}

    /// Runs sensitivity analysis on a forecast by creating a new scenario and running a fresh projection.
    pub fn run_sensitivity_analysis(
        &mut self,
        base_scenario_name: &str,
        new_scenario_name: &str,
        params: &SensitivityParameters,
        historical_transactions: &[Transaction],
    ) -> Result<(), ForecastError> {
        let base_scenario = self.scenarios
            .iter()
            .find(|s| s.name == base_scenario_name)
            .ok_or(ForecastError::ScenarioNotFound)?
            .clone();

        // Create a new scenario based on the base, but with updated parameters
        let mut new_parameters = base_scenario.parameters.clone();
        new_parameters.scenario_parameters.insert(
            "growth_rate", // Corresponds to revenue_growth
            1.0 + (params.revenue_growth / 100.0),
        );
        new_parameters.scenario_parameters.insert(
            "cost_increase", // Corresponds to expense_change
            1.0 + (params.expense_change / 100.0),
        );
        // Note: interest_rate is not directly used in the simple models, but stored for potential use.
        new_parameters.scenario_parameters.insert(
            "interest_rate",
            params.interest_rate,
        );

        self.add_scenario(new_scenario_name.to_string(), new_parameters);

        // Recalculate projections for the new scenario with the full historical data
        self.project_cash_flow(new_scenario_name, historical_transactions)
    }

    /// Updates projections for all scenarios in a forecast
    pub fn update_projections(&mut self, historical_transactions: &[Transaction]) -> Result<(), ForecastError> {
        for scenario in &mut self.scenarios {
            self.project_cash_flow(&scenario.name, historical_transactions)?;
        }
        Ok(())
    }
}