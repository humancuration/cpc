//! Yew components for financial data visualization

use yew::prelude::*;
use crate::domain::{
    budget::Budget,
    savings_goal::SavingsGoal,
};
use cpc_wallet::domain::{
    wallet::{Wallet, WalletTransaction, TransactionType},
};

/// Props for the BudgetChart component
#[derive(Properties, PartialEq)]
pub struct BudgetChartProps {
    pub budget: Budget,
}

/// Component to display a budget chart
#[function_component(BudgetChart)]
pub fn budget_chart(props: &BudgetChartProps) -> Html {
    html! {
        <div class="budget-chart">
            <h3>{ format!("Budget: {}", props.budget.category) }</h3>
            <div class="progress-bar">
                <div 
                    class="progress-fill" 
                    style={format!(
                        "width: {}%; background-color: {}",
                        props.budget.utilization_percentage(),
                        if props.budget.is_over_budget() { "red" } else { "green" }
                    )}
                />
            </div>
            <div class="budget-details">
                <span>{ format!("Allocated: {}", props.budget.allocated_amount.amount) }</span>
                <span>{ format!("Spent: {}", props.budget.spent_amount.amount) }</span>
                <span>{ format!("Remaining: {}", props.budget.remaining_amount().amount) }</span>
            </div>
        </div>
    }
}

/// Props for the SavingsGoalCard component
#[derive(Properties, PartialEq)]
pub struct SavingsGoalCardProps {
    pub goal: SavingsGoal,
}

/// Component to display a savings goal card
#[function_component(SavingsGoalCard)]
pub fn savings_goal_card(props: &SavingsGoalCardProps) -> Html {
    let progress = props.goal.progress_percentage();
    let is_complete = props.goal.is_complete();
    
    html! {
        <div class="savings-goal-card">
            <h3>{ &props.goal.name }</h3>
            <div class="progress-container">
                <div class="progress-bar">
                    <div 
                        class="progress-fill" 
                        style={format!("width: {}%", progress)}
                    />
                </div>
                <span class="progress-text">{ format!("{:.1}%", progress) }</span>
            </div>
            <div class="goal-details">
                <div class="amounts">
                    <span>{ format!("Target: {}", props.goal.target_amount.amount) }</span>
                    <span>{ format!("Current: {}", props.goal.current_amount.amount) }</span>
                </div>
                <div class="dates">
                    <span>{ format!("Target Date: {}", props.goal.target_date.format("%Y-%m-%d")) }</span>
                    <span class={if is_complete { "complete" } else { "days-remaining" }}>
                        { if is_complete { 
                            "Goal Complete!" 
                        } else { 
                            format!("{} days remaining", props.goal.days_until_target()) 
                        } }
                    </span>
                </div>
            </div>
        </div>
    }
}

/// Props for the WalletBalance component
#[derive(Properties, PartialEq)]
pub struct WalletBalanceProps {
    pub wallet: Wallet,
}

/// Component to display wallet balance
#[function_component(WalletBalance)]
pub fn wallet_balance(props: &WalletBalanceProps) -> Html {
    html! {
        <div class="wallet-balance">
            <h3>{ "Wallet Balance" }</h3>
            <div class="balance-amount">
                <span class="amount">{ format!("{} Dabloons", props.wallet.balance.amount) }</span>
            </div>
            <div class="balance-info">
                <span>{ format!("Last updated: {}", props.wallet.updated_at.format("%Y-%m-%d %H:%M")) }</span>
            </div>
        </div>
    }
}

/// Props for the WalletTransactionItem component
#[derive(Properties, PartialEq)]
pub struct WalletTransactionItemProps {
    pub transaction: WalletTransaction,
}

/// Component to display a single wallet transaction
#[function_component(WalletTransactionItem)]
pub fn wallet_transaction_item(props: &WalletTransactionItemProps) -> Html {
    let transaction_type_class = match props.transaction.transaction_type {
        TransactionType::Credit => "credit",
        TransactionType::Debit => "debit",
    };
    
    html! {
        <div class={format!("transaction-item {}", transaction_type_class)}>
            <div class="transaction-details">
                <div class="transaction-amount">
                    <span class="amount">{ format!("{} Dabloons", props.transaction.amount.amount) }</span>
                </div>
                <div class="transaction-meta">
                    <span class="timestamp">{ props.transaction.timestamp.format("%Y-%m-%d %H:%M") }</span>
                    if let Some(description) = &props.transaction.description {
                        <span class="description">{ description }</span>
                    }
                </div>
            </div>
        </div>
    }
}

/// Props for the WalletTransactionHistory component
#[derive(Properties, PartialEq)]
pub struct WalletTransactionHistoryProps {
    pub transactions: Vec<WalletTransaction>,
}

/// Component to display wallet transaction history
#[function_component(WalletTransactionHistory)]
pub fn wallet_transaction_history(props: &WalletTransactionHistoryProps) -> Html {
    html! {
        <div class="transaction-history">
            <h3>{ "Transaction History" }</h3>
            <div class="transactions-list">
                { for props.transactions.iter().map(|transaction| {
                    html! { <WalletTransactionItem transaction={transaction.clone()} /> }
                }) }
            </div>
        </div>
    }
}

/// Props for the WalletOverview component
#[derive(Properties, PartialEq)]
pub struct WalletOverviewProps {
    pub wallet: Wallet,
    pub transactions: Vec<WalletTransaction>,
}

/// Component to display a complete wallet overview
#[function_component(WalletOverview)]
pub fn wallet_overview(props: &WalletOverviewProps) -> Html {
    html! {
        <div class="wallet-overview">
            <h2>{ "Wallet" }</h2>
            
            <div class="wallet-section">
                <WalletBalance wallet={props.wallet.clone()} />
            </div>
            
            <div class="transactions-section">
                <WalletTransactionHistory transactions={props.transactions.clone()} />
            </div>
        </div>
    }
}

/// Props for the FinancialOverview component
#[derive(Properties, PartialEq)]
pub struct FinancialOverviewProps {
    pub budgets: Vec<Budget>,
    pub savings_goals: Vec<SavingsGoal>,
}

/// Component to display a financial overview
#[function_component(FinancialOverview)]
pub fn financial_overview(props: &FinancialOverviewProps) -> Html {
    html! {
        <div class="financial-overview">
            <h2>{ "Financial Overview" }</h2>
            
            <div class="budgets-section">
                <h3>{ "Budgets" }</h3>
                <div class="budgets-grid">
                    { for props.budgets.iter().map(|budget| {
                        html! { <BudgetChart budget={budget.clone()} /> }
                    }) }
                </div>
            </div>
            
            <div class="savings-goals-section">
                <h3>{ "Savings Goals" }</h3>
                <div class="savings-goals-grid">
                    { for props.savings_goals.iter().map(|goal| {
                        html! { <SavingsGoalCard goal={goal.clone()} /> }
                    }) }
                </div>
            </div>
        </div>
    }
}