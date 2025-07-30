//! Currency selector component
//!
//! This component provides a searchable currency dropdown that allows users
//! to select a currency from a list of all supported currencies.

use yew::prelude::*;
use stylist::yew::use_style;
use crate::components::shared::SearchDropdown;
use crate::components::mobile::search_dropdown::MobileSearchDropdown;
use crate::services::currency_api::CurrencyApiService;
use crate::services::mobile::{DeviceSize, get_device_size};
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};

/// Properties for the currency selector component
#[derive(Properties, PartialEq)]
pub struct CurrencySelectorProps {
    /// Callback when a currency is selected
    pub on_select: Callback<Currency>,
    
    /// Optional selected currency
    #[prop_or(None)]
    pub selected: Option<Currency>,
    
    /// ARIA label for accessibility
    #[prop_or("Select currency".to_string())]
    pub aria_label: String,
}

/// State for the currency selector component
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencySelectorState {
    /// List of all available currencies
    currencies: Vec<Currency>,
    
    /// Loading state
    loading: bool,
    
    /// Error state
    error: Option<String>,
}

/// Messages for the currency selector component
#[derive(Debug, Clone)]
pub enum CurrencySelectorMsg {
    /// Set the list of currencies
    SetCurrencies(Vec<Currency>),
    
    /// Set loading state
    SetLoading(bool),
    
    /// Set error state
    SetError(Option<String>),
    
    /// Select a currency
    SelectCurrency(Currency),
}

/// Currency selector component
#[derive(Debug)]
pub struct CurrencySelector {
    /// Component state
    state: CurrencySelectorState,
}

impl Component for CurrencySelector {
    type Message = CurrencySelectorMsg;
    type Properties = CurrencySelectorProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(CurrencySelectorMsg::SetLoading(true));
        Self {
            state: CurrencySelectorState {
                currencies: vec![],
                loading: true,
                error: None,
            },
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            ctx.link().send_future(async {
                match CurrencyApiService::get_all_currencies().await {
                    Ok(currencies) => CurrencySelectorMsg::SetCurrencies(currencies),
                    Err(e) => CurrencySelectorMsg::SetError(Some(format!("Failed to load currencies: {}", e))),
                }
            });
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CurrencySelectorMsg::SetCurrencies(currencies) => {
                self.state.currencies = currencies;
                self.state.loading = false;
                true
            }
            
            CurrencySelectorMsg::SetLoading(loading) => {
                self.state.loading = loading;
                true
            }
            
            CurrencySelectorMsg::SetError(error) => {
                self.state.error = error;
                self.state.loading = false;
                true
            }
            
            CurrencySelectorMsg::SelectCurrency(currency) => {
                ctx.props().on_select.emit(currency);
                true
            }
        }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_styles();
        let device_size = get_device_size();
        
        if self.state.loading {
            return html! {
                <div class={style}>
                    <div class="currency-selector">
                        <p>{"Loading currencies..."}</p>
                    </div>
                </div>
            };
        }
        
        if let Some(error) = &self.state.error {
            return html! {
                <div class={style}>
                    <div class="currency-selector">
                        <p class="error">{"Error: "}{error}</p>
                    </div>
                </div>
            };
        }
        
        let on_select = ctx.link().callback(CurrencySelectorMsg::SelectCurrency);
        
        let item_to_string = Callback::from(|currency: Currency| {
            format!("{} - {} ({})", currency.code(), currency.name, currency.symbol)
        });
        
        // Use mobile-optimized dropdown on mobile devices
        if device_size == DeviceSize::Mobile {
            html! {
                <div class={style}>
                    <div class="currency-selector">
                        <MobileSearchDropdown<Currency>
                            items={self.state.currencies.clone()}
                            on_select={on_select}
                            item_to_string={item_to_string}
                            selected={ctx.props().selected.clone()}
                            placeholder="Search currencies..."
                            aria_label={ctx.props().aria_label.clone()}
                        />
                    </div>
                </div>
            }
        } else {
            html! {
                <div class={style}>
                    <div class="currency-selector">
                        <SearchDropdown<Currency>
                            items={self.state.currencies.clone()}
                            on_select={on_select}
                            item_to_string={item_to_string}
                            selected={ctx.props().selected.clone()}
                            placeholder="Search currencies..."
                            aria_label={ctx.props().aria_label.clone()}
                        />
                    </div>
                </div>
            }
        }
    }
        }
    }
}

/// Get the CSS styles for the component
fn get_styles() -> stylist::Style {
    use_style!(
        r#"
        .currency-selector {
            width: 100%;
        }
        
        .error {
            color: #dc3545;
        }
    "#
    )
}