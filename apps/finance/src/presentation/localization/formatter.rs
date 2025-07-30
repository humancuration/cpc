//! Currency formatter with locale support
//!
//! This module provides functionality for formatting currency amounts according to
//! locale-specific rules for decimal separators, grouping, and currency symbols.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rust_decimal::Decimal;
use crate::domain::currency::{Currency, CurrencyCode};

/// Represents a locale for formatting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Locale {
    /// The locale code (e.g., "en-US", "es-ES")
    pub code: String,
    
    /// The language code (e.g., "en", "es")
    pub language: String,
    
    /// The country code (e.g., "US", "ES")
    pub country: String,
}

impl Locale {
    /// Create a new locale
    pub fn new(code: impl Into<String>, language: impl Into<String>, country: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            language: language.into(),
            country: country.into(),
        }
    }
}

impl From<&str> for Locale {
    fn from(code: &str) -> Self {
        // Simple parsing for common locales
        let parts: Vec<&str> = code.split('-').collect();
        if parts.len() == 2 {
            Self::new(code, parts[0], parts[1])
        } else {
            Self::new(code, code, "")
        }
    }
}

/// Configuration for number formatting
#[derive(Debug, Clone)]
pub struct NumberFormat {
    /// The decimal separator
    pub decimal_separator: char,
    
    /// The thousands separator
    pub thousands_separator: char,
    
    /// The number of decimal places
    pub decimal_places: u32,
}

/// Currency formatter that handles locale-specific formatting
pub struct CurrencyFormatter {
    /// Locale-specific number formatting rules
    number_formats: HashMap<String, NumberFormat>,
    
    /// Default number format
    default_format: NumberFormat,
}

impl CurrencyFormatter {
    /// Create a new currency formatter
    pub fn new() -> Self {
        let mut formatter = Self {
            number_formats: HashMap::new(),
            default_format: NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                decimal_places: 2,
            },
        };
        
        // Initialize common locale formats
        formatter.initialize_common_formats();
        formatter
    }

    /// Initialize common locale formatting rules
    fn initialize_common_formats(&mut self) {
        // US English
        self.number_formats.insert(
            "en-US".to_string(),
            NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                decimal_places: 2,
            },
        );

        // British English
        self.number_formats.insert(
            "en-GB".to_string(),
            NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                decimal_places: 2,
            },
        );

        // German
        self.number_formats.insert(
            "de-DE".to_string(),
            NumberFormat {
                decimal_separator: ',',
                thousands_separator: '.',
                decimal_places: 2,
            },
        );

        // French
        self.number_formats.insert(
            "fr-FR".to_string(),
            NumberFormat {
                decimal_separator: ',',
                thousands_separator: ' ',
                decimal_places: 2,
            },
        );

        // Spanish
        self.number_formats.insert(
            "es-ES".to_string(),
            NumberFormat {
                decimal_separator: ',',
                thousands_separator: '.',
                decimal_places: 2,
            },
        );

        // Japanese
        self.number_formats.insert(
            "ja-JP".to_string(),
            NumberFormat {
                decimal_separator: '.',
                thousands_separator: ',',
                decimal_places: 0, // JPY has no decimal places
            },
        );
    }

    /// Get the number format for a locale
    fn get_number_format(&self, locale: &Locale) -> &NumberFormat {
        self.number_formats
            .get(&locale.code)
            .unwrap_or(&self.default_format)
    }

    /// Format a decimal number according to locale rules
    pub fn format_number(&self, number: Decimal, locale: &Locale, decimal_places: u32) -> String {
        let format = self.get_number_format(locale);
        
        // Convert to string with the specified decimal places
        let formatted = format!("{:.*}", decimal_places as usize, number);
        
        // Split into integer and fractional parts
        let parts: Vec<&str> = formatted.split('.').collect();
        let integer_part = parts[0];
        let fractional_part = if parts.len() > 1 { parts[1] } else { "" };
        
        // Add thousands separators to integer part
        let separated_integer = self.add_thousands_separators(integer_part, format.thousands_separator);
        
        // Combine parts with locale-specific decimal separator
        if decimal_places > 0 && !fractional_part.is_empty() {
            format!("{}{}{}", separated_integer, format.decimal_separator, fractional_part)
        } else {
            separated_integer
        }
    }

    /// Add thousands separators to an integer string
    fn add_thousands_separators(&self, integer: &str, separator: char) -> String {
        let clean_integer = integer.replace("-", "");
        let mut result = String::new();
        let chars: Vec<char> = clean_integer.chars().collect();
        let len = chars.len();
        
        for (i, ch) in chars.iter().enumerate() {
            result.push(*ch);
            // Add separator every 3 digits from the right
            if (len - i - 1) % 3 == 0 && i != len - 1 {
                result.push(separator);
            }
        }
        
        // Add back negative sign if needed
        if integer.starts_with('-') {
            format!("-{}", result)
        } else {
            result
        }
    }

    /// Format a currency amount according to locale rules
    pub fn format_currency(&self, amount: Decimal, currency: &Currency, locale: &Locale) -> String {
        let format = self.get_number_format(locale);
        let decimal_places = currency.decimal_places;
        
        let formatted_number = self.format_number(amount, locale, decimal_places);
        
        // Format according to locale conventions
        match locale.code.as_str() {
            // European style: symbol after amount
            "de-DE" | "fr-FR" | "es-ES" => {
                format!("{} {}", formatted_number, currency.symbol)
            }
            // Default style: symbol before amount
            _ => {
                format!("{}{}", currency.symbol, formatted_number)
            }
        }
    }

    /// Format a currency amount with ISO code for disambiguation
    pub fn format_currency_with_code(&self, amount: Decimal, currency: &Currency, locale: &Locale) -> String {
        let formatted = self.format_currency(amount, currency, locale);
        format!("{} ({})", formatted, currency.code())
    }
}

impl Default for CurrencyFormatter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_locale_creation() {
        let locale: Locale = "en-US".into();
        assert_eq!(locale.code, "en-US");
        assert_eq!(locale.language, "en");
        assert_eq!(locale.country, "US");
    }

    #[test]
    fn test_number_formatting() {
        let formatter = CurrencyFormatter::new();
        let locale: Locale = "en-US".into();
        
        let result = formatter.format_number(dec!(1234.56), &locale, 2);
        assert_eq!(result, "1,234.56");
        
        let result = formatter.format_number(dec!(1234567.89), &locale, 2);
        assert_eq!(result, "1,234,567.89");
    }

    #[test]
    fn test_german_formatting() {
        let formatter = CurrencyFormatter::new();
        let locale: Locale = "de-DE".into();
        
        let result = formatter.format_number(dec!(1234.56), &locale, 2);
        assert_eq!(result, "1.234,56");
    }

    #[test]
    fn test_currency_formatting() {
        let formatter = CurrencyFormatter::new();
        let locale: Locale = "en-US".into();
        let currency: Currency = CurrencyCode::new("USD").into();
        
        let result = formatter.format_currency(dec!(1234.56), &currency, &locale);
        assert_eq!(result, "$1,234.56");
    }

    #[test]
    fn test_european_currency_formatting() {
        let formatter = CurrencyFormatter::new();
        let locale: Locale = "de-DE".into();
        let currency: Currency = CurrencyCode::new("EUR").into();
        
        let result = formatter.format_currency(dec!(1234.56), &currency, &locale);
        assert_eq!(result, "1.234,56 €");
    }

    #[test]
    fn test_japanese_yen_formatting() {
        let formatter = CurrencyFormatter::new();
        let locale: Locale = "ja-JP".into();
        let currency: Currency = CurrencyCode::new("JPY").into();
        
        let result = formatter.format_currency(dec!(1234), &currency, &locale);
        assert_eq!(result, "¥1,234");
    }
}