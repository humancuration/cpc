//! Exchange rate manager component
//!
//! This component provides an interface for managing currency exchange rates,
//! including viewing rates, filtering, manual overrides, and refreshing rates.

use yew::prelude::*;
use stylist::yew::use_style;
use crate::services::currency_api::CurrencyApiService;
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};
use chrono::{DateTime, Utc};

/// Properties for the exchange rate manager component
#[derive(Properties, PartialEq)]
pub struct ExchangeRateManagerProps {
    // No props needed for this component
}

/// State for the exchange rate manager component
#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeRateManagerState {
    /// List of all exchange rates
    exchange_rates: Vec<ExchangeRateEntry>,
    
    /// Filtered exchange rates
    filtered_rates: Vec<ExchangeRateEntry>,
    
    /// Loading state
    loading: bool,
    
    /// Error state
    error: Option<String>,
    
    /// Filter criteria
    filter_from_currency: Option<String>,
    filter_to_currency: Option<String>,
    filter_provider: Option<String>,
    
    /// Available currencies for filtering
    available_currencies: Vec<String>,
    
    /// Available providers for filtering
    available_providers: Vec<String>,
}

/// Exchange rate entry
#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeRateEntry {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    pub last_updated: DateTime<Utc>,
    pub provider: String,
    pub is_active: bool,
}

/// Messages for the exchange rate manager component
#[derive(Debug, Clone)]
pub enum ExchangeRateManagerMsg {
    /// Set exchange rates
    SetExchangeRates(Vec<ExchangeRateEntry>),
    
    /// Set available currencies
    SetAvailableCurrencies(Vec<String>),
    
    /// Set available providers
    SetAvailableProviders(Vec<String>),
    
    /// Set loading state
    SetLoading(bool),
    
    /// Set error state
    SetError(Option<String>),
    
    /// Set filter for from currency
    SetFilterFromCurrency(Option<String>),
    
    /// Set filter for to currency
    SetFilterToCurrency(Option<String>),
    
    /// Set filter for provider
    SetFilterProvider(Option<String>),
    
    /// Apply filters
    ApplyFilters,
    
    /// Refresh rates
    RefreshRates,
    
    /// Override rate
    OverrideRate(String, String, f64),
}

/// Exchange rate manager component
#[derive(Debug)]
pub struct ExchangeRateManager {
    /// Component state
    state: ExchangeRateManagerState,
}

impl Component for ExchangeRateManager {
    type Message = ExchangeRateManagerMsg;
    type Properties = ExchangeRateManagerProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ExchangeRateManagerMsg::SetLoading(true));
        
        Self {
            state: ExchangeRateManagerState {
                exchange_rates: vec![],
                filtered_rates: vec![],
                loading: true,
                error: None,
                filter_from_currency: None,
                filter_to_currency: None,
                filter_provider: None,
                available_currencies: vec![],
                available_providers: vec![],
            },
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Load initial data
            ctx.link().send_message(ExchangeRateManagerMsg::RefreshRates);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ExchangeRateManagerMsg::SetExchangeRates(rates) => {
                self.state.exchange_rates = rates;
                self.state.filtered_rates = self.state.exchange_rates.clone();
                ctx.link().send_message(ExchangeRateManagerMsg::ApplyFilters);
                true
            }
            
            ExchangeRateManagerMsg::SetAvailableCurrencies(currencies) => {
                self.state.available_currencies = currencies;
                true
            }
            
            ExchangeRateManagerMsg::SetAvailableProviders(providers) => {
                self.state.available_providers = providers;
                true
            }
            
            ExchangeRateManagerMsg::SetLoading(loading) => {
                self.state.loading = loading;
                true
            }
            
            ExchangeRateManagerMsg::SetError(error) => {
                self.state.error = error;
                self.state.loading = false;
                true
            }
            
            ExchangeRateManagerMsg::SetFilterFromCurrency(currency) => {
                self.state.filter_from_currency = currency;
                true
            }
            
            ExchangeRateManagerMsg::SetFilterToCurrency(currency) => {
                self.state.filter_to_currency = currency;
                true
            }
            
            ExchangeRateManagerMsg::SetFilterProvider(provider) => {
                self.state.filter_provider = provider;
                true
            }
            
            ExchangeRateManagerMsg::ApplyFilters => {
                let mut filtered = self.state.exchange_rates.clone();
                
                if let Some(from_currency) = &self.state.filter_from_currency {
                    filtered.retain(|rate| rate.from_currency == *from_currency);
                }
                
                if let Some(to_currency) = &self.state.filter_to_currency {
                    filtered.retain(|rate| rate.to_currency == *to_currency);
                }
                
                if let Some(provider) = &self.state.filter_provider {
                    filtered.retain(|rate| rate.provider == *provider);
                }
                
                self.state.filtered_rates = filtered;
                true
            }
            
            ExchangeRateManagerMsg::RefreshRates => {
                ctx.link().send_message(ExchangeRateManagerMsg::SetLoading(true));
                ctx.link().send_message(ExchangeRateManagerMsg::SetError(None));
                
                ctx.link().send_future(async {
                    // In a real implementation, this would fetch from the API
                    // For now, we'll create mock data
                    let rates = vec![
                        ExchangeRateEntry {
                            from_currency: "USD".to_string(),
                            to_currency: "EUR".to_string(),
                            rate: 0.85,
                            last_updated: Utc::now(),
                            provider: "ECB".to_string(),
                            is_active: true,
                        },
                        ExchangeRateEntry {
                            from_currency: "USD".to_string(),
                            to_currency: "GBP".to_string(),
                            rate: 0.73,
                            last_updated: Utc::now(),
                            provider: "ECB".to_string(),
                            is_active: true,
                        },
                        ExchangeRateEntry {
                            from_currency: "EUR".to_string(),
                            to_currency: "USD".to_string(),
                            rate: 1.18,
                            last_updated: Utc::now(),
                            provider: "ECB".to_string(),
                            is_active: true,
                        },
                        ExchangeRateEntry {
                            from_currency: "GBP".to_string(),
                            to_currency: "USD".to_string(),
                            rate: 1.37,
                            last_updated: Utc::now(),
                            provider: "ECB".to_string(),
                            is_active: true,
                        },
                    ];
                    
                    ExchangeRateManagerMsg::SetExchangeRates(rates)
                });
                true
            }
            
            ExchangeRateManagerMsg::OverrideRate(from, to, rate) => {
                // In a real implementation, this would call the API to override the rate
                // For now, we'll just show a message
                web_sys::console::log_1(&format!("Override rate: {} to {} = {}", from, to, rate).into());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_styles();
        
        let on_filter_from_change = ctx.link().callback(|e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            ExchangeRateManagerMsg::SetFilterFromCurrency(if value.is_empty() { None } else { Some(value) })
        });
        
        let on_filter_to_change = ctx.link().callback(|e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            ExchangeRateManagerMsg::SetFilterToCurrency(if value.is_empty() { None } else { Some(value) })
        });
        
        let on_filter_provider_change = ctx.link().callback(|e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            ExchangeRateManagerMsg::SetFilterProvider(if value.is_empty() { None } else { Some(value) })
        });
        
        let on_apply_filters = ctx.link().callback(|_| ExchangeRateManagerMsg::ApplyFilters);
        let on_refresh = ctx.link().callback(|_| ExchangeRateManagerMsg::RefreshRates);
        
        html! {
            <div class={style}>
                <div class="exchange-rate-manager">
                    <div class="manager-header">
                        <h3>{"Exchange Rate Manager"}</h3>
                        <button onclick={on_refresh} disabled={self.state.loading}>
                            {"Refresh Rates"}
                        </button>
                    </div>
                    
                    <div class="filters">
                        <div class="filter-group">
                            <label>{"From Currency"}</label>
                            <select onchange={on_filter_from_change}>
                                <option value="">{ "All" }</option>
                                {for self.state.available_currencies.iter().map(|currency| {
                                    html! { <option value={currency.clone()}>{currency}</option> }
                                })}
                            </select>
                        </div>
                        
                        <div class="filter-group">
                            <label>{"To Currency"}</label>
                            <select onchange={on_filter_to_change}>
                                <option value="">{ "All" }</option>
                                {for self.state.available_currencies.iter().map(|currency| {
                                    html! { <option value={currency.clone()}>{currency}</option> }
                                })}
                            </select>
                        </div>
                        
                        <div class="filter-group">
                            <label>{"Provider"}</label>
                            <select onchange={on_filter_provider_change}>
                                <option value="">{ "All" }</option>
                                {for self.state.available_providers.iter().map(|provider| {
                                    html! { <option value={provider.clone()}>{provider}</option> }
                                })}
                            </select>
                        </div>
                        
                        <div class="filter-group">
                            <button onclick={on_apply_filters}>{"Apply Filters"}</button>
                        </div>
                    </div>
                    
                    if self.state.loading {
                        <div class="loading">
                            <p>{"Loading exchange rates..."}</p>
                        </div>
                    }
                    
                    if let Some(error) = &self.state.error {
                        <div class="error">
                            <p>{error}</p>
                        </div>
                    }
                    
                    <div class="rates-table-container">
                        <table class="rates-table">
                            <thead>
                                <tr>
                                    <th>{"From"}</th>
                                    <th>{"To"}</th>
                                    <th>{"Rate"}</th>
                                    <th>{"Last Updated"}</th>
                                    <th>{"Provider"}</th>
                                    <th>{"Status"}</th>
                                    <th>{"Actions"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for self.state.filtered_rates.iter().map(|rate| {
                                    let on_override = ctx.link().callback(move |_| {
                                        ExchangeRateManagerMsg::OverrideRate(
                                            rate.from_currency.clone(),
                                            rate.to_currency.clone(),
                                            rate.rate
                                        )
                                    });
                                    
                                    html! {
                                        <tr>
                                            <td>{&rate.from_currency}</td>
                                            <td>{&rate.to_currency}</td>
                                            <td>{format!("{:.6}", rate.rate)}</td>
                                            <td>{rate.last_updated.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                                            <td>{&rate.provider}</td>
                                            <td>
                                                <span class={if rate.is_active { "status-active" } else { "status-inactive" }}>
                                                    {if rate.is_active { "Active" } else { "Inactive" }}
                                                </span>
                                            </td>
                                            <td>
                                                <button onclick={on_override}>{"Override"}</button>
                                            </td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="audit-trail">
                        <h4>{"Audit Trail"}</h4>
                        <p>{"Recent rate changes will appear here..."}</p>
                    </div>
                </div>
            </div>
        }
    }
}

/// Get the CSS styles for the component
fn get_styles() -> stylist::Style {
    use_style!(
        r#"
        .exchange-rate-manager {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 8px;
            background-color: #f9f9f9;
        }
        
        .manager-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 20px;
        }
        
        .manager-header h3 {
            margin: 0;
        }
        
        .manager-header button {
            padding: 8px 16px;
            background-color: #007bff;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        
        .manager-header button:hover:not(:disabled) {
            background-color: #0056b3;
        }
        
        .manager-header button:disabled {
            background-color: #ccc;
            cursor: not-allowed;
        }
        
        .filters {
            display: flex;
            flex-wrap: wrap;
            gap: 15px;
            margin-bottom: 20px;
            padding: 20px;
            background-color: white;
            border-radius: 4px;
            border: 1px solid #eee;
        }
        
        .filter-group {
            display: flex;
            flex-direction: column;
        }
        
        .filter-group label {
            margin-bottom: 5px;
            font-weight: bold;
        }
        
        .filter-group select,
        .filter-group button {
            padding: 8px 12px;
            border: 1px solid #ccc;
            border-radius: 4px;
        }
        
        .filter-group button {
            background-color: #28a745;
            color: white;
            cursor: pointer;
        }
        
        .filter-group button:hover {
            background-color: #218838;
        }
        
        .loading, .error {
            padding: 15px;
            border-radius: 4px;
            margin: 20px 0;
            text-align: center;
        }
        
        .loading {
            background-color: #e9ecef;
        }
        
        .error {
            background-color: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }
        
        .rates-table-container {
            overflow-x: auto;
            margin-bottom: 30px;
        }
        
        .rates-table {
            width: 100%;
            border-collapse: collapse;
            background-color: white;
            border-radius: 4px;
            overflow: hidden;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }
        
        .rates-table th,
        .rates-table td {
            padding: 12px 15px;
            text-align: left;
            border-bottom: 1px solid #eee;
        }
        
        .rates-table th {
            background-color: #f8f9fa;
            font-weight: 600;
        }
        
        .rates-table tr:last-child td {
            border-bottom: none;
        }
        
        .rates-table tr:hover {
            background-color: #f5f5f5;
        }
        
        .status-active {
            color: #28a745;
            font-weight: bold;
        }
        
        .status-inactive {
            color: #dc3545;
            font-weight: bold;
        }
        
        .rates-table button {
            padding: 6px 12px;
            background-color: #ffc107;
            color: #212529;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        }
        
        .rates-table button:hover {
            background-color: #e0a800;
        }
        
        .audit-trail {
            background-color: white;
            padding: 20px;
            border-radius: 4px;
            border: 1px solid #eee;
        }
        
        .audit-trail h4 {
            margin-top: 0;
            margin-bottom: 15px;
        }
        
        .audit-trail p {
            color: #666;
            font-style: italic;
        }
    "#
    )
}