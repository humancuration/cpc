//! Currency registry
//!
//! This module provides a comprehensive registry of all ISO 4217 currencies
//! with their properties and metadata.

use std::collections::HashMap;
use super::{Currency, CurrencyCode};

/// Registry of all supported currencies
pub struct CurrencyRegistry {
    currencies: HashMap<String, Currency>,
}

impl CurrencyRegistry {
    /// Create a new currency registry with all ISO 4217 currencies
    pub fn new() -> Self {
        let mut registry = Self {
            currencies: HashMap::new(),
        };
        
        // Populate with all ISO 4217 currencies
        registry.populate_currencies();
        registry
    }

    /// Get a currency by its code
    pub fn get(&self, code: &str) -> Option<&Currency> {
        self.currencies.get(&code.to_uppercase())
    }

    /// Get all currencies
    pub fn all(&self) -> Vec<&Currency> {
        self.currencies.values().collect()
    }

    /// Check if a currency exists in the registry
    pub fn contains(&self, code: &str) -> bool {
        self.currencies.contains_key(&code.to_uppercase())
    }

    /// Populate the registry with all ISO 4217 currencies
    fn populate_currencies(&mut self) {
        let currencies = vec![
            // Major world currencies
            ("USD", "United States Dollar", "$", 2),
            ("EUR", "Euro", "€", 2),
            ("GBP", "British Pound", "£", 2),
            ("JPY", "Japanese Yen", "¥", 0),
            ("CAD", "Canadian Dollar", "CA$", 2),
            ("AUD", "Australian Dollar", "A$", 2),
            ("CHF", "Swiss Franc", "CHF", 2),
            ("CNY", "Chinese Yuan", "¥", 2),
            ("SEK", "Swedish Krona", "kr", 2),
            ("NZD", "New Zealand Dollar", "NZ$", 2),
            ("MXN", "Mexican Peso", "$", 2),
            ("SGD", "Singapore Dollar", "S$", 2),
            ("HKD", "Hong Kong Dollar", "HK$", 2),
            ("NOK", "Norwegian Krone", "kr", 2),
            ("KRW", "South Korean Won", "₩", 0),
            ("TRY", "Turkish Lira", "₺", 2),
            ("RUB", "Russian Ruble", "₽", 2),
            ("INR", "Indian Rupee", "₹", 2),
            ("BRL", "Brazilian Real", "R$", 2),
            ("ZAR", "South African Rand", "R", 2),
            
            // Additional currencies
            ("AED", "United Arab Emirates Dirham", "د.إ", 2),
            ("AFN", "Afghan Afghani", "؋", 2),
            ("ALL", "Albanian Lek", "L", 2),
            ("AMD", "Armenian Dram", "֏", 2),
            ("ANG", "Netherlands Antillean Guilder", "ƒ", 2),
            ("AOA", "Angolan Kwanza", "Kz", 2),
            ("ARS", "Argentine Peso", "$", 2),
            ("AWG", "Aruban Florin", "ƒ", 2),
            ("AZN", "Azerbaijani Manat", "₼", 2),
            ("BAM", "Bosnia-Herzegovina Convertible Mark", "KM", 2),
            ("BBD", "Barbadian Dollar", "$", 2),
            ("BDT", "Bangladeshi Taka", "৳", 2),
            ("BGN", "Bulgarian Lev", "лв", 2),
            ("BHD", "Bahraini Dinar", "د.ب", 3),
            ("BIF", "Burundian Franc", "FBu", 0),
            ("BMD", "Bermudan Dollar", "$", 2),
            ("BND", "Brunei Dollar", "$", 2),
            ("BOB", "Bolivian Boliviano", "Bs.", 2),
            ("BSD", "Bahamian Dollar", "$", 2),
            ("BTN", "Bhutanese Ngultrum", "Nu.", 2),
            ("BWP", "Botswanan Pula", "P", 2),
            ("BYN", "Belarusian Ruble", "Br", 2),
            ("BZD", "Belize Dollar", "$", 2),
            ("CDF", "Congolese Franc", "FC", 2),
            ("CLP", "Chilean Peso", "$", 0),
            ("COP", "Colombian Peso", "$", 2),
            ("CRC", "Costa Rican Colón", "₡", 2),
            ("CUP", "Cuban Peso", "$", 2),
            ("CVE", "Cape Verdean Escudo", "$", 2),
            ("CZK", "Czech Koruna", "Kč", 2),
            ("DJF", "Djiboutian Franc", "Fdj", 0),
            ("DKK", "Danish Krone", "kr", 2),
            ("DOP", "Dominican Peso", "RD$", 2),
            ("DZD", "Algerian Dinar", "د.ج", 2),
            ("EGP", "Egyptian Pound", "£", 2),
            ("ERN", "Eritrean Nakfa", "Nfk", 2),
            ("ETB", "Ethiopian Birr", "Br", 2),
            ("FJD", "Fijian Dollar", "$", 2),
            ("FKP", "Falkland Islands Pound", "£", 2),
            ("GEL", "Georgian Lari", "₾", 2),
            ("GHS", "Ghanaian Cedi", "₵", 2),
            ("GIP", "Gibraltar Pound", "£", 2),
            ("GMD", "Gambian Dalasi", "D", 2),
            ("GNF", "Guinean Franc", "FG", 0),
            ("GTQ", "Guatemalan Quetzal", "Q", 2),
            ("GYD", "Guyanaese Dollar", "$", 2),
            ("HNL", "Honduran Lempira", "L", 2),
            ("HRK", "Croatian Kuna", "kn", 2),
            ("HTG", "Haitian Gourde", "G", 2),
            ("HUF", "Hungarian Forint", "Ft", 2),
            ("IDR", "Indonesian Rupiah", "Rp", 2),
            ("ILS", "Israeli Shekel", "₪", 2),
            ("IQD", "Iraqi Dinar", "ع.د", 3),
            ("IRR", "Iranian Rial", "﷼", 2),
            ("ISK", "Icelandic Króna", "kr", 0),
            ("JMD", "Jamaican Dollar", "$", 2),
            ("JOD", "Jordanian Dinar", "د.ا", 3),
            ("KES", "Kenyan Shilling", "Sh", 2),
            ("KGS", "Kyrgystani Som", "с", 2),
            ("KHR", "Cambodian Riel", "៛", 2),
            ("KMF", "Comorian Franc", "CF", 0),
            ("KPW", "North Korean Won", "₩", 2),
            ("KWD", "Kuwaiti Dinar", "د.ك", 3),
            ("KYD", "Cayman Islands Dollar", "$", 2),
            ("KZT", "Kazakhstani Tenge", "₸", 2),
            ("LAK", "Laotian Kip", "₭", 2),
            ("LBP", "Lebanese Pound", "ل.ل", 2),
            ("LKR", "Sri Lankan Rupee", "₨", 2),
            ("LRD", "Liberian Dollar", "$", 2),
            ("LSL", "Lesotho Loti", "L", 2),
            ("LYD", "Libyan Dinar", "ل.د", 3),
            ("MAD", "Moroccan Dirham", "د.م.", 2),
            ("MDL", "Moldovan Leu", "L", 2),
            ("MGA", "Malagasy Ariary", "Ar", 2),
            ("MKD", "Macedonian Denar", "ден", 2),
            ("MMK", "Myanmar Kyat", "Ks", 2),
            ("MNT", "Mongolian Tugrik", "₮", 2),
            ("MOP", "Macanese Pataca", "MOP$", 2),
            ("MRU", "Mauritanian Ouguiya", "UM", 2),
            ("MUR", "Mauritian Rupee", "₨", 2),
            ("MVR", "Maldivian Rufiyaa", "Rf", 2),
            ("MWK", "Malawian Kwacha", "MK", 2),
            ("MYR", "Malaysian Ringgit", "RM", 2),
            ("MZN", "Mozambican Metical", "MT", 2),
            ("NAD", "Namibian Dollar", "$", 2),
            ("NGN", "Nigerian Naira", "₦", 2),
            ("NIO", "Nicaraguan Córdoba", "C$", 2),
            ("NPR", "Nepalese Rupee", "₨", 2),
            ("OMR", "Omani Rial", "ر.ع.", 3),
            ("PAB", "Panamanian Balboa", "B/.", 2),
            ("PEN", "Peruvian Sol", "S/", 2),
            ("PGK", "Papua New Guinean Kina", "K", 2),
            ("PHP", "Philippine Peso", "₱", 2),
            ("PKR", "Pakistani Rupee", "₨", 2),
            ("PLN", "Polish Zloty", "zł", 2),
            ("PYG", "Paraguayan Guarani", "₲", 0),
            ("QAR", "Qatari Rial", "ر.ق", 2),
            ("RON", "Romanian Leu", "lei", 2),
            ("RSD", "Serbian Dinar", "дин", 2),
            ("RWF", "Rwandan Franc", "FRw", 0),
            ("SAR", "Saudi Riyal", "ر.س", 2),
            ("SBD", "Solomon Islands Dollar", "$", 2),
            ("SCR", "Seychellois Rupee", "₨", 2),
            ("SDG", "Sudanese Pound", "ج.س.", 2),
            ("SHP", "Saint Helena Pound", "£", 2),
            ("SLL", "Sierra Leonean Leone", "Le", 2),
            ("SOS", "Somali Shilling", "Sh", 2),
            ("SRD", "Surinamese Dollar", "$", 2),
            ("SSP", "South Sudanese Pound", "£", 2),
            ("STN", "São Tomé and Príncipe Dobra", "Db", 2),
            ("SYP", "Syrian Pound", "£", 2),
            ("SZL", "Swazi Lilangeni", "L", 2),
            ("THB", "Thai Baht", "฿", 2),
            ("TJS", "Tajikistani Somoni", "SM", 2),
            ("TMT", "Turkmenistani Manat", "T", 2),
            ("TND", "Tunisian Dinar", "د.ت", 3),
            ("TOP", "Tongan Pa'anga", "T$", 2),
            ("TTD", "Trinidad and Tobago Dollar", "$", 2),
            ("TWD", "New Taiwan Dollar", "NT$", 2),
            ("TZS", "Tanzanian Shilling", "Sh", 2),
            ("UAH", "Ukrainian Hryvnia", "₴", 2),
            ("UGX", "Ugandan Shilling", "UGX", 0),
            ("UYU", "Uruguayan Peso", "$", 0),
            ("UZS", "Uzbekistan Som", "so'm", 2),
            ("VES", "Venezuelan Bolívar", "Bs.", 2),
            ("VND", "Vietnamese Dong", "₫", 0),
            ("VUV", "Vanuatu Vatu", "VT", 0),
            ("WST", "Samoan Tala", "WS$", 2),
            ("XAF", "CFA Franc BEAC", "FCFA", 0),
            ("XCD", "East Caribbean Dollar", "$", 2),
            ("XOF", "CFA Franc BCEAO", "CFA", 0),
            ("XPF", "CFP Franc", "₣", 0),
            ("YER", "Yemeni Rial", "﷼", 2),
            ("ZMW", "Zambian Kwacha", "ZK", 2),
            ("ZWL", "Zimbabwean Dollar", "$", 2),
        ];

        // Add traditional currencies
        for (code, name, symbol, decimal_places) in currencies {
            let currency = Currency::traditional(
                CurrencyCode::new(code),
                name.to_string(),
                symbol.to_string(),
                decimal_places,
            );
            self.currencies.insert(code.to_string(), currency);
        }

        // Add Dabloons
        let dabloon = Currency::dabloon();
        self.currencies.insert("DABLOONS".to_string(), dabloon);
    }
}

impl Default for CurrencyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = CurrencyRegistry::new();
        assert!(!registry.currencies.is_empty());
    }

    #[test]
    fn test_get_currency() {
        let registry = CurrencyRegistry::new();
        let usd = registry.get("USD");
        assert!(usd.is_some());
        assert_eq!(usd.unwrap().code(), "USD");
    }

    #[test]
    fn test_contains_currency() {
        let registry = CurrencyRegistry::new();
        assert!(registry.contains("EUR"));
        assert!(!registry.contains("XYZ"));
    }

    #[test]
    fn test_all_currencies() {
        let registry = CurrencyRegistry::new();
        let all = registry.all();
        assert!(!all.is_empty());
    }

    #[test]
    fn test_dabloon_currency() {
        let registry = CurrencyRegistry::new();
        let dabloon = registry.get("DABLOONS");
        assert!(dabloon.is_some());
        assert!(dabloon.unwrap().is_dabloon());
    }
}