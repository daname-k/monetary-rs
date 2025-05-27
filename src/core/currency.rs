use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::OnceLock;
use crate::core::CurrencyUnit;
use crate::errors::CurrencyError;



// Currency representation - wrapper around CurrencyUnit with display logic
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    unit: CurrencyUnit,
    symbol: String,
}

impl Currency {
    pub fn new(unit: CurrencyUnit, symbol: &str) -> Self {
        Self {
            unit,
            symbol: symbol.to_string(),
        }
    }

    // Delegate core properties to the underlying unit
    pub fn code(&self) -> &str { 
        self.unit.get_code() 
    }
    
    pub fn numeric_code(&self) -> i32 { 
        self.unit.get_numeric_code() 
    }
    
    pub fn precision(&self) -> i32 { 
        self.unit.get_default_fraction_digits() 
    }
    
    pub fn display_name(&self) -> &str { 
        self.unit.get_display_name() 
    }
    
    pub fn symbol(&self) -> &str { 
        &self.symbol 
    }
    
    pub fn get_unit(&self) -> &CurrencyUnit {
        &self.unit
    }

    // Create from ISO code - looks up from registry
    pub fn from_code(code: &str) -> Option<Self> {
    get_currency_registry().get(&code.to_uppercase() as &str).cloned()
    }


    // Create currency with custom symbol (override default)
    pub fn with_symbol(code: &str, symbol: &str) -> Option<Self> {
        if let Some(base_currency) = Self::from_code(code) {
            Some(Self::new(base_currency.unit.clone(), symbol))
        } else {
            None
        }
    }
}

// Common currencies - static constructors
impl Currency {
    pub fn usd() -> Self {
        let unit = CurrencyUnit::new("USD", 840, 2, "US Dollar");
        Self::new(unit, "$")
    }

    pub fn eur() -> Self {
        let unit = CurrencyUnit::new("EUR", 978, 2, "Euro");
        Self::new(unit, "€")
    }

    pub fn gbp() -> Self {
        let unit = CurrencyUnit::new("GBP", 826, 2, "British Pound Sterling");
        Self::new(unit, "£")
    }

    pub fn jpy() -> Self {
        let unit = CurrencyUnit::new("JPY", 392, 0, "Japanese Yen");
        Self::new(unit, "¥")
    }

    pub fn chf() -> Self {
        let unit = CurrencyUnit::new("CHF", 756, 2, "Swiss Franc");
        Self::new(unit, "Fr")
    }

    pub fn cad() -> Self {
        let unit = CurrencyUnit::new("CAD", 124, 2, "Canadian Dollar");
        Self::new(unit, "C$")
    }

    pub fn aud() -> Self {
        let unit = CurrencyUnit::new("AUD", 36, 2, "Australian Dollar");
        Self::new(unit, "A$")
    }

    pub fn cny() -> Self {
        let unit = CurrencyUnit::new("CNY", 156, 2, "Chinese Yuan");
        Self::new(unit, "¥")
    }

    pub fn inr() -> Self {
        let unit = CurrencyUnit::new("INR", 356, 2, "Indian Rupee");
        Self::new(unit, "₹")
    }

    pub fn krw() -> Self {
        let unit = CurrencyUnit::new("KRW", 410, 0, "South Korean Won");
        Self::new(unit, "₩")
    }

    pub fn brl() -> Self {
        let unit = CurrencyUnit::new("BRL", 986, 2, "Brazilian Real");
        Self::new(unit, "R$")
    }

    pub fn rub() -> Self {
        let unit = CurrencyUnit::new("RUB", 643, 2, "Russian Ruble");
        Self::new(unit, "₽")
    }

    pub fn zar() -> Self {
        let unit = CurrencyUnit::new("ZAR", 710, 2, "South African Rand");
        Self::new(unit, "R")
    }

    pub fn mxn() -> Self {
        let unit = CurrencyUnit::new("MXN", 484, 2, "Mexican Peso");
        Self::new(unit, "$")
    }

    pub fn sgd() -> Self {
        let unit = CurrencyUnit::new("SGD", 702, 2, "Singapore Dollar");
        Self::new(unit, "S$")
    }

    pub fn nok() -> Self {
        let unit = CurrencyUnit::new("NOK", 578, 2, "Norwegian Krone");
        Self::new(unit, "kr")
    }

    pub fn sek() -> Self {
        let unit = CurrencyUnit::new("SEK", 752, 2, "Swedish Krona");
        Self::new(unit, "kr")
    }

    pub fn dkk() -> Self {
        let unit = CurrencyUnit::new("DKK", 208, 2, "Danish Krone");
        Self::new(unit, "kr")
    }

    pub fn pln() -> Self {
        let unit = CurrencyUnit::new("PLN", 985, 2, "Polish Zloty");
        Self::new(unit, "zł")
    }

    pub fn czk() -> Self {
        let unit = CurrencyUnit::new("CZK", 203, 2, "Czech Koruna");
        Self::new(unit, "Kč")
    }

    pub fn huf() -> Self {
        let unit = CurrencyUnit::new("HUF", 348, 2, "Hungarian Forint");
        Self::new(unit, "Ft")
    }

    pub fn ils() -> Self {
        let unit = CurrencyUnit::new("ILS", 376, 2, "Israeli New Shekel");
        Self::new(unit, "₪")
    }

    pub fn aed() -> Self {
        let unit = CurrencyUnit::new("AED", 784, 2, "UAE Dirham");
        Self::new(unit, "د.إ")
    }

    pub fn sar() -> Self {
        let unit = CurrencyUnit::new("SAR", 682, 2, "Saudi Riyal");
        Self::new(unit, "﷼")
    }

    pub fn try_currency() -> Self {
        let unit = CurrencyUnit::new("TRY", 949, 2, "Turkish Lira");
        Self::new(unit, "₺")
    }

    // Cryptocurrencies (non-ISO)
    pub fn btc() -> Self {
        let unit = CurrencyUnit::new("BTC", 0, 8, "Bitcoin");
        Self::new(unit, "₿")
    }

    pub fn eth() -> Self {
        let unit = CurrencyUnit::new("ETH", 0, 18, "Ethereum");
        Self::new(unit, "Ξ")
    }

    pub fn ltc() -> Self {
        let unit = CurrencyUnit::new("LTC", 0, 8, "Litecoin");
        Self::new(unit, "Ł")
    }

    // Precious metals (XAU, XAG are ISO codes)
    pub fn xau() -> Self {
        let unit = CurrencyUnit::new("XAU", 959, 4, "Gold (troy ounce)");
        Self::new(unit, "Au")
    }

    pub fn xag() -> Self {
        let unit = CurrencyUnit::new("XAG", 961, 4, "Silver (troy ounce)");
        Self::new(unit, "Ag")
    }
}

// Global currency registry for lookup by code
static CURRENCY_REGISTRY: OnceLock<HashMap<&'static str, Currency>> = OnceLock::new();

fn get_currency_registry() -> &'static HashMap<&'static str, Currency> {
    CURRENCY_REGISTRY.get_or_init(|| {
        let mut registry = HashMap::new();
        
        // Major currencies
        registry.insert("USD", Currency::usd());
        registry.insert("EUR", Currency::eur());
        registry.insert("GBP", Currency::gbp());
        registry.insert("JPY", Currency::jpy());
        registry.insert("CHF", Currency::chf());
        registry.insert("CAD", Currency::cad());
        registry.insert("AUD", Currency::aud());
        registry.insert("CNY", Currency::cny());
        registry.insert("INR", Currency::inr());
        registry.insert("KRW", Currency::krw());
        registry.insert("BRL", Currency::brl());
        registry.insert("RUB", Currency::rub());
        registry.insert("ZAR", Currency::zar());
        registry.insert("MXN", Currency::mxn());
        registry.insert("SGD", Currency::sgd());
        registry.insert("NOK", Currency::nok());
        registry.insert("SEK", Currency::sek());
        registry.insert("DKK", Currency::dkk());
        registry.insert("PLN", Currency::pln());
        registry.insert("CZK", Currency::czk());
        registry.insert("HUF", Currency::huf());
        registry.insert("ILS", Currency::ils());
        registry.insert("AED", Currency::aed());
        registry.insert("SAR", Currency::sar());
        registry.insert("TRY", Currency::try_currency());
        
        // Cryptocurrencies  
        registry.insert("BTC", Currency::btc());
        registry.insert("ETH", Currency::eth());
        registry.insert("LTC", Currency::ltc());
        
        // Precious metals
        registry.insert("XAU", Currency::xau());
        registry.insert("XAG", Currency::xag());
        
        registry
    })
}

// Display formatting
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

// String parsing - allows "USD", "EUR", etc.
impl FromStr for Currency {
    type Err = CurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_code(s).ok_or_else(|| CurrencyError::unknown_currency(s))
    }
}

// Utility functions
impl Currency {
    /// Get all available currencies
    pub fn available_currencies() -> Vec<&'static Currency> {
        get_currency_registry().values().collect()
    }

    /// Check if a currency code is supported
    pub fn is_supported(code: &str) -> bool {
        get_currency_registry().contains_key(code.to_uppercase().as_str())
    }

    /// Get currency by numeric code
    pub fn from_numeric_code(numeric_code: i32) -> Option<Self> {
        get_currency_registry()
            .values()
            .find(|currency| currency.numeric_code() == numeric_code)
            .cloned()
    }

    /// Compare currencies (by numeric code for performance)
    pub fn same_currency(&self, other: &Currency) -> bool {
        self.numeric_code() == other.numeric_code()
    }

    /// Check if this is a cryptocurrency
    pub fn is_cryptocurrency(&self) -> bool {
        matches!(self.code(), "BTC" | "ETH" | "LTC")
    }

    /// Check if this is a precious metal
    pub fn is_precious_metal(&self) -> bool {
        matches!(self.code(), "XAU" | "XAG")
    }

    /// Check if this is fiat currency
    pub fn is_fiat(&self) -> bool {
        !self.is_cryptocurrency() && !self.is_precious_metal()
    }

    /// Get formatted display with symbol
    pub fn format_with_symbol(&self, show_code: bool) -> String {
        if show_code {
            format!("{} {}", self.symbol(), self.code())
        } else {
            self.symbol().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        let usd = Currency::usd();
        assert_eq!(usd.code(), "USD");
        assert_eq!(usd.numeric_code(), 840);
        assert_eq!(usd.precision(), 2);
        assert_eq!(usd.symbol(), "$");
    }

    #[test]
    fn test_currency_from_code() {
        let eur = Currency::from_code("EUR").unwrap();
        assert_eq!(eur.code(), "EUR");
        assert_eq!(eur.symbol(), "€");

        let invalid = Currency::from_code("INVALID");
        assert!(invalid.is_none());
    }

    #[test]
    fn test_currency_from_str() {
        let gbp: Currency = "GBP".parse().unwrap();
        assert_eq!(gbp.code(), "GBP");
        assert_eq!(gbp.symbol(), "£");

        let invalid: Result<Currency, _> = "INVALID".parse();
        assert!(invalid.is_err());
    }

    #[test]
    fn test_currency_comparison() {
        let usd1 = Currency::usd();
        let usd2 = Currency::usd();
        let eur = Currency::eur();

        assert!(usd1.same_currency(&usd2));
        assert!(!usd1.same_currency(&eur));
    }

    #[test]
    fn test_currency_types() {
        let usd = Currency::usd();
        let btc = Currency::btc();
        let xau = Currency::xau();

        assert!(usd.is_fiat());
        assert!(!usd.is_cryptocurrency());
        assert!(!usd.is_precious_metal());

        assert!(btc.is_cryptocurrency());
        assert!(!btc.is_fiat());
        assert!(!btc.is_precious_metal());

        assert!(xau.is_precious_metal());
        assert!(!xau.is_fiat());
        assert!(!xau.is_cryptocurrency());
    }

    #[test]
    fn test_currency_registry() {
        assert!(Currency::is_supported("USD"));
        assert!(Currency::is_supported("EUR"));
        assert!(!Currency::is_supported("INVALID"));

        let usd_by_numeric = Currency::from_numeric_code(840).unwrap();
        assert_eq!(usd_by_numeric.code(), "USD");
    }

    #[test]
    fn test_custom_symbol() {
        let custom_usd = Currency::with_symbol("USD", "US$").unwrap();
        assert_eq!(custom_usd.code(), "USD");
        assert_eq!(custom_usd.symbol(), "US$");
    }

    #[test]
    fn test_formatting() {
        let usd = Currency::usd();
        assert_eq!(format!("{}", usd), "USD");
        assert_eq!(usd.format_with_symbol(true), "$ USD");
        assert_eq!(usd.format_with_symbol(false), "$");
    }
}