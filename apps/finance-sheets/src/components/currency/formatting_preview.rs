//! Currency formatting preview component
//!
//! This component provides a visual demonstration of how currency values
//! are formatted based on locale and user preferences.

use yew::prelude::*;
use stylist::yew::use_style;
use crate::components::shared::SearchDropdown;
use crate::services::currency_api::CurrencyApiService;
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};

/// Properties for the formatting preview component
#[derive(Properties, PartialEq)]
pub struct FormattingPreviewProps {
    // No props needed for this component
}

/// State for the formatting preview component
#[derive(Debug, Clone, PartialEq)]
pub struct FormattingPreviewState {
    /// Selected currency for preview
    selected_currency: Option<Currency>,
    
    /// Selected locale for formatting
    selected_locale: String,
    
    /// List of available locales
    available_locales: Vec<String>,
    
    /// Formatted examples
    formatted_examples: Vec<(String, String)>, // (value, formatted)
    
    /// Loading state
    loading: bool,
    
    /// Error state
    error: Option<String>,
    
    /// Whether to show currency symbols or codes
    show_symbols: bool,
}

/// Messages for the formatting preview component
#[derive(Debug, Clone)]
pub enum FormattingPreviewMsg {
    /// Set the selected currency
    SetCurrency(Currency),
    
    /// Set the selected locale
    SetLocale(String),
    
    /// Set available locales
    SetAvailableLocales(Vec<String>),
    
    /// Set formatted examples
    SetFormattedExamples(Vec<(String, String)>),
    
    /// Set loading state
    SetLoading(bool),
    
    /// Set error state
    SetError(Option<String>),
    
    /// Toggle between symbols and codes
    ToggleSymbols(bool),
    
    /// Refresh the preview
    RefreshPreview,
}

/// Formatting preview component
#[derive(Debug)]
pub struct FormattingPreview {
    /// Component state
    state: FormattingPreviewState,
}

impl Component for FormattingPreview {
    type Message = FormattingPreviewMsg;
    type Properties = FormattingPreviewProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(FormattingPreviewMsg::SetLoading(true));
        
        // Initialize with some default locales
        let available_locales = vec![
            "en-US".to_string(),
            "en-GB".to_string(),
            "de-DE".to_string(),
            "fr-FR".to_string(),
            "ja-JP".to_string(),
            "zh-CN".to_string(),
        ];
        
        Self {
            state: FormattingPreviewState {
                selected_currency: None,
                selected_locale: "en-US".to_string(),
                available_locales,
                formatted_examples: vec![],
                loading: true,
                error: None,
                show_symbols: true,
            },
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Load initial data
            ctx.link().send_message(FormattingPreviewMsg::RefreshPreview);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormattingPreviewMsg::SetCurrency(currency) => {
                self.state.selected_currency = Some(currency);
                ctx.link().send_message(FormattingPreviewMsg::RefreshPreview);
                true
            }
            
            FormattingPreviewMsg::SetLocale(locale) => {
                self.state.selected_locale = locale;
                ctx.link().send_message(FormattingPreviewMsg::RefreshPreview);
                true
            }
            
            FormattingPreviewMsg::SetAvailableLocales(locales) => {
                self.state.available_locales = locales;
                true
            }
            
            FormattingPreviewMsg::SetFormattedExamples(examples) => {
                self.state.formatted_examples = examples;
                self.state.loading = false;
                true
            }
            
            FormattingPreviewMsg::SetLoading(loading) => {
                self.state.loading = loading;
                true
            }
            
            FormattingPreviewMsg::SetError(error) => {
                self.state.error = error;
                self.state.loading = false;
                true
            }
            
            FormattingPreviewMsg::ToggleSymbols(show_symbols) => {
                self.state.show_symbols = show_symbols;
                ctx.link().send_message(FormattingPreviewMsg::RefreshPreview);
                true
            }
            
            FormattingPreviewMsg::RefreshPreview => {
                if let Some(currency) = &self.state.selected_currency {
                    let currency_code = currency.code().to_string();
                    let locale = self.state.selected_locale.clone();
                    let show_symbols = self.state.show_symbols;
                    
                    ctx.link().send_message(FormattingPreviewMsg::SetLoading(true));
                    ctx.link().send_message(FormattingPreviewMsg::SetError(None));
                    
                    ctx.link().send_future(async move {
                        // Example values to format
                        let values = vec!["1.00", "100.00", "1000.00", "1000000.00"];
                        let mut examples = Vec::new();
                        
                        for value in values {
                            match CurrencyApiService::format_currency(
                                value.parse().unwrap_or_default(),
                                &currency_code,
                                &locale,
                                show_symbols,
                            ).await {
                                Ok(formatted) => examples.push((value.to_string(), formatted)),
                                Err(e) => return FormattingPreviewMsg::SetError(Some(format!("Formatting failed: {}", e))),
                            }
                        }
                        
                        FormattingPreviewMsg::SetFormattedExamples(examples)
                    });
                } else {
                    // Show default examples if no currency selected
                    let examples = vec![
                        ("1.00".to_string(), "Select a currency to preview".to_string()),
                        ("100.00".to_string(), "Select a currency to preview".to_string()),
                        ("1000.00".to_string(), "Select a currency to preview".to_string()),
                        ("1000000.00".to_string(), "Select a currency to preview".to_string()),
                    ];
                    ctx.link().send_message(FormattingPreviewMsg::SetFormattedExamples(examples));
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_styles();
        
        let on_currency_select = ctx.link().callback(FormattingPreviewMsg::SetCurrency);
        let on_locale_select = ctx.link().callback(FormattingPreviewMsg::SetLocale);
        let on_toggle_symbols = ctx.link().callback(FormattingPreviewMsg::ToggleSymbols);
        
        let item_to_string = Callback::from(|currency: Currency| {
            format!("{} - {} ({})", currency.code(), currency.name, currency.symbol)
        });
        
        let locale_item_to_string = Callback::from(|locale: String| locale.clone());
        
        html! {
            <div class={style}>
                <div class="formatting-preview">
                    <div class="preview-header">
                        <h3>{"Currency Formatting Preview"}</h3>
                    </div>
                    
                    <div class="preview-controls">
                        <div class="control-group">
                            <label>{"Currency"}</label>
                            <CurrencySelector 
                                on_select={on_currency_select}
                                selected={self.state.selected_currency.clone()}
                                aria_label="Select currency for preview"
                            />
                        </div>
                        
                        <div class="control-group">
                            <label>{"Locale"}</label>
                            <SearchDropdown<String>
                                items={self.state.available_locales.clone()}
                                on_select={on_locale_select}
                                item_to_string={locale_item_to_string}
                                selected={Some(self.state.selected_locale.clone())}
                                placeholder="Search locales..."
                                aria_label="Select locale for formatting"
                            />
                        </div>
                        
                        <div class="control-group">
                            <label>
                                <input
                                    type="checkbox"
                                    checked={self.state.show_symbols}
                                    onchange={ctx.link().callback(|e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        FormattingPreviewMsg::ToggleSymbols(input.checked())
                                    })}
                                />
                                {" Show symbols instead of codes"}
                            </label>
                        </div>
                    </div>
                    
                    if self.state.loading {
                        <div class="loading">
                            <p>{"Loading preview..."}</p>
                        </div>
                    }
                    
                    if let Some(error) = &self.state.error {
                        <div class="error">
                            <p>{error}</p>
                        </div>
                    }
                    
                    <div class="preview-examples">
                        <h4>{"Formatting Examples"}</h4>
                        <table>
                            <thead>
                                <tr>
                                    <th>{"Value"}</th>
                                    <th>{"Formatted"}</th>
                                </tr>
                            </thead>
                            <tbody>
                                {for self.state.formatted_examples.iter().map(|(value, formatted)| {
                                    html! {
                                        <tr>
                                            <td>{value}</td>
                                            <td class="formatted-value">{formatted}</td>
                                        </tr>
                                    }
                                })}
                            </tbody>
                        </table>
                    </div>
                    
                    <div class="formatting-info">
                        <h4>{"Formatting Rules"}</h4>
                        <ul>
                            <li>{"Decimal separator: "}{self.get_decimal_separator()}</li>
                            <li>{"Thousand separator: "}{self.get_thousand_separator()}</li>
                            <li>{"Symbol position: "}{self.get_symbol_position()}</li>
                            <li>{"Negative format: "}{self.get_negative_format()}</li>
                        </ul>
                    </div>
                </div>
            </div>
        }
    }
}

impl FormattingPreview {
    /// Get the decimal separator for the current locale
    fn get_decimal_separator(&self) -> &'static str {
        match self.state.selected_locale.as_str() {
            "de-DE" | "fr-FR" | "es-ES" => ",",
            _ => ".",
        }
    }
    
    /// Get the thousand separator for the current locale
    fn get_thousand_separator(&self) -> &'static str {
        match self.state.selected_locale.as_str() {
            "de-DE" => ".",
            "fr-FR" => " ",
            _ => ",",
        }
    }
    
    /// Get the symbol position for the current locale
    fn get_symbol_position(&self) -> &'static str {
        match self.state.selected_locale.as_str() {
            "en-GB" | "de-DE" | "fr-FR" => "prefix",
            _ => "prefix",
        }
    }
    
    /// Get the negative format for the current locale
    fn get_negative_format(&self) -> &'static str {
        match self.state.selected_locale.as_str() {
            "de-DE" => "-1.000,00 €",
            "fr-FR" => "-1 000,00 €",
            _ => "-$1,000.00",
        }
    }
}

/// Get the CSS styles for the component
fn get_styles() -> stylist::Style {
    use_style!(
        r#"
        .formatting-preview {
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
            border: 1px solid #ddd;
            border-radius: 8px;
            background-color: #f9f9f9;
        }
        
        .preview-header h3 {
            margin-top: 0;
            text-align: center;
        }
        
        .preview-controls {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
            padding: 20px;
            background-color: white;
            border-radius: 4px;
            border: 1px solid #eee;
        }
        
        .control-group {
            display: flex;
            flex-direction: column;
        }
        
        .control-group label {
            margin-bottom: 5px;
            font-weight: bold;
        }
        
        .control-group input[type="checkbox"] {
            margin-right: 8px;
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
        
        .preview-examples {
            margin-bottom: 30px;
        }
        
        .preview-examples h4 {
            margin-top: 0;
            margin-bottom: 15px;
        }
        
        .preview-examples table {
            width: 100%;
            border-collapse: collapse;
            background-color: white;
            border-radius: 4px;
            overflow: hidden;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        }
        
        .preview-examples th,
        .preview-examples td {
            padding: 12px 15px;
            text-align: left;
            border-bottom: 1px solid #eee;
        }
        
        .preview-examples th {
            background-color: #f8f9fa;
            font-weight: 600;
        }
        
        .preview-examples tr:last-child td {
            border-bottom: none;
        }
        
        .formatted-value {
            font-family: monospace;
            font-weight: bold;
        }
        
        .formatting-info {
            background-color: white;
            padding: 20px;
            border-radius: 4px;
            border: 1px solid #eee;
        }
        
        .formatting-info h4 {
            margin-top: 0;
            margin-bottom: 15px;
        }
        
        .formatting-info ul {
            margin: 0;
            padding-left: 20px;
        }
        
        .formatting-info li {
            margin-bottom: 8px;
        }
    "#
    )
}