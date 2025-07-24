//! Simple ledger example demonstrating the accounting system

use cpc_core::accounting::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ledger
    let mut ledger = Ledger::new("My Business".to_string(), "USD".to_string());
    
    // Create some accounts
    let cash_account = Account::new(
        "1000".to_string(),
        "Cash".to_string(),
        AccountType::Asset,
        None,
        "USD",
    );
    
    let revenue_account = Account::new(
        "4000".to_string(),
        "Sales Revenue".to_string(),
        AccountType::Revenue,
        None,
        "USD",
    );
    
    // Add accounts to ledger
    ledger.add_account(cash_account.clone())?;
    ledger.add_account(revenue_account.clone())?;
    
    // Create a transaction: $1000 in sales
    let mut transaction = Transaction::new(
        Utc::now(),
        "Sales for the day".to_string(),
        Some("INV-001".to_string()),
        vec!["sales".to_string()],
    );
    
    // Add journal entries
    transaction.add_entry(JournalEntry::new(
        cash_account.id,
        EntryType::Debit,
        Money::new(1000.0, "USD"),
        Some("Cash received from sales".to_string()),
    ))?;
    
    transaction.add_entry(JournalEntry::new(
        revenue_account.id,
        EntryType::Credit,
        Money::new(1000.0, "USD"),
        Some("Sales revenue".to_string()),
    ))?;
    
    // Record the transaction
    ledger.record_transaction(transaction)?;
    
    // Print results
    println!("Ledger: {}", ledger.name);
    println!("Cash balance: ${}", ledger.get_balance(cash_account.id)?.to_float());
    
    let income_statement = ledger.get_income_statement()?;
    println!("Net income: ${}", income_statement.net_income.to_float());
    
    let trial_balance = ledger.get_trial_balance()?;
    println!("Trial balance is balanced: {}", trial_balance.is_balanced);
    
    Ok(())
}