//! Currency converter component
//!
//! This component provides a real-time currency conversion tool with
//! two currency selectors and an amount input field.

use yew::prelude::*;
use stylist::yew::use_style;
use crate::components::currency::CurrencySelector;
use crate::services::currency_api::CurrencyApiService;
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};
use rust_decimal::Decimal;
use std::str::FromStr;

/// Properties for the currency converter component
#[derive(Properties, PartialEq)]
pub struct CurrencyConverterProps {
    // No props needed for this component
}

/// State for the currency converter component
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyConverterState {
    /// Amount to convert
    amount: String,
    
    /// Source currency
    from_currency: Option<Currency>,
    
    /// Target currency
    to_currency: Option<Currency>,
    
    /// Converted amount
    converted_amount: Option<String>,
    
    /// Last updated timestamp
    last_updated: Option<String>,
    
    /// Loading state
    loading: bool,
    
    /// Error state
    error: Option<String>,
}

/// Messages for the currency converter component
#[derive(Debug, Clone)]
pub enum CurrencyConverterMsg {
    /// Set the amount to convert
    SetAmount(String),
    
    /// Set the source currency
    SetFromCurrency(Currency),
    
    /// Set the target currency
    SetToCurrency(Currency),
    
    /// Set the converted amount
    SetConvertedAmount(Option<String>),
    
    /// Set loading state
    SetLoading(bool),
    
    /// Set error state
    SetError(Option<String>),
    
    /// Set last updated timestamp
    SetLastUpdated(Option<String>),
    
    /// Convert the currency
    Convert,
}

/// Currency converter component
#[derive(Debug)]
pub struct CurrencyConverter {
    /// Component state
    state: CurrencyConverterState,
}

impl Component for CurrencyConverter {
    type Message = CurrencyConverterMsg;
    type Properties = CurrencyConverterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: CurrencyConverterState {
                amount: "1.00".to_string(),
                from_currency: None,
                to_currency: None,
                converted_amount: None,
                last_updated: None,
                loading: false,
                error: None,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CurrencyConverterMsg::SetAmount(amount) => {
                self.state.amount = amount;
                // Auto-convert when amount changes
                if self.state.from_currency.is_some() && self.state.to_currency.is_some() {
                    ctx.link().send_message(CurrencyConverterMsg::Convert);
                }
                true
            }
            
            CurrencyConverterMsg::SetFromCurrency(currency) => {
                self.state.from_currency = Some(currency);
                // Auto-convert when currency changes
                if !self.state.amount.is_empty() && self.state.to_currency.is_some() {
                    ctx.link().send_message(CurrencyConverterMsg::Convert);
                }
                true
            }
            
            CurrencyConverterMsg::SetToCurrency(currency) => {
                self.state.to_currency = Some(currency);
                // Auto-convert when currency changes
                if !self.state.amount.is_empty() && self.state.from_currency.is_some() {
                    ctx.link().send_message(CurrencyConverterMsg::Convert);
                }
                true
            }
            
            CurrencyConverterMsg::SetConvertedAmount(amount) => {
                self.state.converted_amount = amount;
                true
            }
            
            CurrencyConverterMsg::SetLoading(loading) => {
                self.state.loading = loading;
                true
            }
            
            CurrencyConverterMsg::SetError(error) => {
                self.state.error = error;
                true
            }
            
            CurrencyConverterMsg::SetLastUpdated(timestamp) => {
                self.state.last_updated = timestamp;
                true
            }
            
            CurrencyConverterMsg::Convert => {
                // Validate inputs
                if self.state.amount.is_empty() {
                    ctx.link().send_message(CurrencyConverterMsg::SetError(Some("Please enter an amount".to_string())));
                    return true;
                }
                
                if self.state.from_currency.is_none() || self.state.to_currency.is_none() {
                    ctx.link().send_message(CurrencyConverterMsg::SetError(Some("Please select both currencies".to_string())));
                    return true;
                }
                
                // Parse amount
                let amount = match Decimal::from_str(&self.state.amount) {
                    Ok(amount) => amount,
                    Err(_) => {
                        ctx.link().send_message(CurrencyConverterMsg::SetError(Some("Invalid amount".to_string())));
                        return true;
                    }
                };
                
                let from_currency = self.state.from_currency.as_ref().unwrap().code().to_string();
                let to_currency = self.state.to_currency.as_ref().unwrap().code().to_string();
                
                // Set loading state
                ctx.link().send_message(CurrencyConverterMsg::SetLoading(true));
                ctx.link().send_message(CurrencyConverterMsg::SetError(None));
                
                // Perform conversion
                ctx.link().send_future(async move {
                    match CurrencyApiService::convert_currency(amount, &from_currency, &to_currency).await {
                        Ok(converted) => {
                            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
                            CurrencyConverterMsg::SetConvertedAmount(Some(converted.to_string()))
                        },
                        Err(e) => CurrencyConverterMsg::SetError(Some(format!("Conversion failed: {}", e))),
                    }
                });
                
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_styles();
        
        let on_amount_input = ctx.link().callback(|e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            CurrencyConverterMsg::SetAmount(input.value())
        });
        
        let on_from_currency_select = ctx.link().callback(CurrencyConverterMsg::SetFromCurrency);
        let on_to_currency_select = ctx.link().callback(CurrencyConverterMsg::SetToCurrency);
        
        html! {
            <div class={style}>
                <div class="currency-converter">
                    <div class="converter-header">
                        <h3>{"Currency Converter"}</h3>
                    </div>
                    
                    <div class="converter-form">
                        <div class="amount-input">
                            <label for="amount">{"Amount"}</label>
                            <input
                                type="text"
                                id="amount"
                                value={self.state.amount.clone()}
                                oninput={on_amount_input}
                                placeholder="Enter amount"
                                disabled={self.state.loading}
                            />
                        </div>
                        
                        <div class="currency-selectors">
                            <div class="from-currency">
                                <label>{"From"}</label>
                                <CurrencySelector 
                                    on_select={on_from_currency_select}
                                    selected={self.state.from_currency.clone()}
                                    aria_label="Source currency"
                                />
                            </div>
                            
                            <div class="swap-button">
                                <button 
                                    onclick={ctx.link().callback(|_| CurrencyConverterMsg::Convert)}
                                    disabled={self.state.loading}
                                    aria-label="Convert currencies"
                                >
                                    {"Convert"}
                                </button>
                            </div>
                            
                            <div class="to-currency">
                                <label>{"To"}</label>
                                <CurrencySelector 
                                    on_select={on_to_currency_select}
                                    selected={self.state.to_currency.clone()}
                                    aria_label="Target currency"
                                />
                            </div>
                        </div>
                        
                        if self.state.loading {
                            <div class="loading">
                                <p>{"Converting..."}</p>
                            </div>
                        }
                        
                        if let Some(error) = &self.state.error {
                            <div class="error">
                                <p>{error}</p>
                            </div>
                        }
                        
                        if let Some(converted) = &self.state.converted_amount {
                            <div class="result">
                                <div class="converted-amount">
                                    <span class="amount">{converted}</span>
                                    if let Some(currency) = &self.state.to_currency {
                                        <span class="currency-code">{currency.code()}</span>
                                    }
                                </div>
                                if let Some(timestamp) = &self.state.last_updated {
                                    <div class="last-updated">
                                        <small>{"Last updated: "}{timestamp}</small>
                                    </div>
                                }
                            </div>
                        }
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
        .currency-converter {
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 8px;
            background-color: #f9f9f9;
        }
        
        .converter-header h3 {
            margin-top: 0;
            text-align: center;
        }
        
        .amount-input {
            margin-bottom: 20px;
        }
        
        .amount-input label {
            display: block;
            margin-bottom: 5px;
            font-weight: bold;
        }
        
        .amount-input input {
            width: 100%;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 4px;
            font-size: 16px;
            box-sizing: border-box;
        }
        
        .currency-selectors {
            display: grid;
            grid-template-columns: 1fr auto 1fr;
            gap: 15px;
            align-items: end;
            margin-bottom: 20px;
        }
        
        .from-currency, .to-currency {
            display: flex;
            flex-direction: column;
        }
        
        .from-currency label, .to-currency label {
            margin-bottom: 5px;
            font-weight: bold;
        }
        
        .swap-button {
            display: flex;
            align-items: end;
            justify-content: center;
        }
        
        .swap-button button {
            padding: 10px 15px;
            background-color: #007bff;
            color: white;
            border: none;
            border-radius: 4px;
            cursor: pointer;
            font-size: 14px;
        }
        
        .swap-button button:hover:not(:disabled) {
            background-color: #0056b3;
        }
        
        .swap-button button:disabled {
            background-color: #ccc;
            cursor: not-allowed;
        }
        
        .loading, .error, .result {
            padding: 15px;
            border-radius: 4px;
            margin-top: 15px;
        }
        
        .loading {
            background-color: #e9ecef;
            text-align: center;
        }
        
        .error {
            background-color: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }
        
        .result {
            background-color: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
        }
        
        .converted-amount {
            display: flex;
            align-items: baseline;
            gap: 10px;
            font-size: 24px;
            font-weight: bold;
        }
        
        .amount {
            font-size: 28px;
        }
        
        .currency-code {
            font-size: 20px;
            color: #666;
        }
        
        .last-updated {
            margin-top: 10px;
            text-align: right;
        }
        
        .last-updated small {
            color: #666;
        }
        
        @media (max-width: 768px) {
            .currency-selectors {
                grid-template-columns: 1fr;
            }
            
            .swap-button {
                margin: 10px 0;
            }
        }
    "#
    )
}