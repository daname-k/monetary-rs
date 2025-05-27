use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
use crate::core::currency::Currency;
use crate::errors::CurrencyError;

/// Money enum representing different currencies with their values
/// Values are stored as floating-point numbers in major currency units (e.g., dollars for USD)
#[derive(Debug, Clone, PartialEq)]
pub enum Money {
    // Major Fiat Currencies
    USD(f64),  // dollars
    EUR(f64),  // euros
    GBP(f64),  // pounds
    JPY(f64),  // yen
    CHF(f64),  // francs
    CAD(f64),  // dollars
    AUD(f64),  // dollars
    CNY(f64),  // yuan
    INR(f64),  // rupees
    KRW(f64),  // won
    BRL(f64),  // reais
    RUB(f64),  // rubles
    ZAR(f64),  // rand
    MXN(f64),  // pesos
    SGD(f64),  // dollars
    
    // European Currencies
    NOK(f64),  // kroner
    SEK(f64),  // kronor
    DKK(f64),  // kroner
    PLN(f64),  // zloty
    CZK(f64),  // koruny
    HUF(f64),  // forint
    
    // Middle East / Africa
    ILS(f64),  // shekels
    AED(f64),  // dirhams
    SAR(f64),  // riyals
    TRY(f64),  // lira
    
    // Cryptocurrencies
    BTC(f64),  // bitcoins
    ETH(f64),  // ether
    LTC(f64),  // litecoins
    
    // Precious Metals
    XAU(f64),  // troy ounces
    XAG(f64),  // troy ounces
}

impl Money {
    /// Create Money from amount in major currency units
    pub fn new(currency_code: &str, amount: f64) -> Result<Self, CurrencyError> {
        match currency_code.to_uppercase().as_str() {
            "USD" => Ok(Money::USD(amount)),
            "EUR" => Ok(Money::EUR(amount)),
            "GBP" => Ok(Money::GBP(amount)),
            "JPY" => Ok(Money::JPY(amount)),
            "CHF" => Ok(Money::CHF(amount)),
            "CAD" => Ok(Money::CAD(amount)),
            "AUD" => Ok(Money::AUD(amount)),
            "CNY" => Ok(Money::CNY(amount)),
            "INR" => Ok(Money::INR(amount)),
            "KRW" => Ok(Money::KRW(amount)),
            "BRL" => Ok(Money::BRL(amount)),
            "RUB" => Ok(Money::RUB(amount)),
            "ZAR" => Ok(Money::ZAR(amount)),
            "MXN" => Ok(Money::MXN(amount)),
            "SGD" => Ok(Money::SGD(amount)),
            "NOK" => Ok(Money::NOK(amount)),
            "SEK" => Ok(Money::SEK(amount)),
            "DKK" => Ok(Money::DKK(amount)),
            "PLN" => Ok(Money::PLN(amount)),
            "CZK" => Ok(Money::CZK(amount)),
            "HUF" => Ok(Money::HUF(amount)),
            "ILS" => Ok(Money::ILS(amount)),
            "AED" => Ok(Money::AED(amount)),
            "SAR" => Ok(Money::SAR(amount)),
            "TRY" => Ok(Money::TRY(amount)),
            "BTC" => Ok(Money::BTC(amount)),
            "ETH" => Ok(Money::ETH(amount)),
            "LTC" => Ok(Money::LTC(amount)),
            "XAU" => Ok(Money::XAU(amount)),
            "XAG" => Ok(Money::XAG(amount)),
            _ => Err(CurrencyError::unknown_currency(currency_code.to_string())),
        }
    }
    
    /// Create Money from minor units (cents, pence, etc.)
    pub fn from_minor_units(currency_code: &str, minor_units: i64) -> Result<Self, CurrencyError> {
        let currency = Currency::from_code(currency_code)
            .ok_or_else(|| CurrencyError::unknown_currency(currency_code.to_string()))?;
        
        let precision = currency.precision();
        let divisor = 10_f64.powi(precision);
        let amount = minor_units as f64 / divisor;
        
        Self::new(currency_code, amount)
    }
    
    /// Get the currency code for this Money variant
    pub fn currency_code(&self) -> &'static str {
        match self {
            Money::USD(_) => "USD",
            Money::EUR(_) => "EUR",
            Money::GBP(_) => "GBP",
            Money::JPY(_) => "JPY",
            Money::CHF(_) => "CHF",
            Money::CAD(_) => "CAD",
            Money::AUD(_) => "AUD",
            Money::CNY(_) => "CNY",
            Money::INR(_) => "INR",
            Money::KRW(_) => "KRW",
            Money::BRL(_) => "BRL",
            Money::RUB(_) => "RUB",
            Money::ZAR(_) => "ZAR",
            Money::MXN(_) => "MXN",
            Money::SGD(_) => "SGD",
            Money::NOK(_) => "NOK",
            Money::SEK(_) => "SEK",
            Money::DKK(_) => "DKK",
            Money::PLN(_) => "PLN",
            Money::CZK(_) => "CZK",
            Money::HUF(_) => "HUF",
            Money::ILS(_) => "ILS",
            Money::AED(_) => "AED",
            Money::SAR(_) => "SAR",
            Money::TRY(_) => "TRY",
            Money::BTC(_) => "BTC",
            Money::ETH(_) => "ETH",
            Money::LTC(_) => "LTC",
            Money::XAU(_) => "XAU",
            Money::XAG(_) => "XAG",
        }
    }
    
    /// Get the raw amount value
    pub fn amount(&self) -> f64 {
        match self {
            Money::USD(v) | Money::EUR(v) | Money::GBP(v) | Money::JPY(v) |
            Money::CHF(v) | Money::CAD(v) | Money::AUD(v) | Money::CNY(v) |
            Money::INR(v) | Money::KRW(v) | Money::BRL(v) | Money::RUB(v) |
            Money::ZAR(v) | Money::MXN(v) | Money::SGD(v) | Money::NOK(v) |
            Money::SEK(v) | Money::DKK(v) | Money::PLN(v) | Money::CZK(v) |
            Money::HUF(v) | Money::ILS(v) | Money::AED(v) | Money::SAR(v) |
            Money::TRY(v) | Money::BTC(v) | Money::ETH(v) | Money::LTC(v) |
            Money::XAU(v) | Money::XAG(v) => *v,
        }
    }
    
    /// Convert to minor units (cents, pence, etc.) as integer
    pub fn to_minor_units(&self) -> i64 {
        let currency = self.currency();
        let precision = currency.precision();
        let multiplier = 10_f64.powi(precision);
        (self.amount() * multiplier).round() as i64
    }
    
    /// Get the Currency struct for this Money
    pub fn currency(&self) -> Currency {
        Currency::from_code(self.currency_code()).unwrap()
    }


    /// Check if same currency
    pub fn same_currency(&self, other: &Money) -> bool {
        self.currency_code() == other.currency_code()
    }
    
    /// Zero value for the currency
    pub fn zero(currency_code: &str) -> Result<Self, CurrencyError> {
        Self::new(currency_code, 0.0)
    }
    
    /// Check if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.amount().abs() < f64::EPSILON
    }
    
    /// Check if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.amount() > f64::EPSILON
    }
    
    /// Check if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.amount() < -f64::EPSILON
    }
    
    /// Get absolute value
    pub fn abs(&self) -> Self {
        let abs_amount = self.amount().abs();
        Self::new(self.currency_code(), abs_amount).unwrap()
    }
    
    /// Round to specified decimal places
    pub fn round(&self, decimal_places: u32) -> Self {
        let multiplier = 10_f64.powi(decimal_places as i32);
        let rounded = (self.amount() * multiplier).round() / multiplier;
        Self::new(self.currency_code(), rounded).unwrap()
    }
    
    /// Round to currency's default precision
    pub fn round_to_precision(&self) -> Self {
        let precision = self.currency().precision() as u32;
        self.round(precision)
    }
}

// Convenient constructors for common amounts
impl Money {
    pub fn usd(dollars: f64) -> Self { Money::USD(dollars) }
    pub fn eur(euros: f64) -> Self { Money::EUR(euros) }
    pub fn gbp(pounds: f64) -> Self { Money::GBP(pounds) }
    pub fn jpy(yen: f64) -> Self { Money::JPY(yen) }
    pub fn chf(francs: f64) -> Self { Money::CHF(francs) }
    pub fn cad(dollars: f64) -> Self { Money::CAD(dollars) }
    pub fn aud(dollars: f64) -> Self { Money::AUD(dollars) }
    pub fn cny(yuan: f64) -> Self { Money::CNY(yuan) }
    pub fn btc(bitcoins: f64) -> Self { Money::BTC(bitcoins) }
    pub fn eth(ether: f64) -> Self { Money::ETH(ether) }
}

// Arithmetic operations (only between same currencies)
impl Add for Money {
    type Output = Result<Money, CurrencyError>;
    
    fn add(self, other: Money) -> Self::Output {
        if !self.same_currency(&other) {
            return Err(CurrencyError::currency_mismatch(
                 self.currency_code().to_string(),
                 other.currency_code().to_string()));
        }
        
        let result_amount = self.amount() + other.amount();
        Money::new(self.currency_code(), result_amount)
    }
}

impl Sub for Money {
    type Output = Result<Money, CurrencyError>;
    
    fn sub(self, other: Money) -> Self::Output {
        if !self.same_currency(&other) {
            return Err(CurrencyError::currency_mismatch(
                 self.currency_code().to_string(),
                 other.currency_code().to_string(),
            ));
        }
        
        let result_amount = self.amount() - other.amount();
        Money::new(self.currency_code(), result_amount)
    }
}

// Scalar multiplication
impl Mul<f64> for Money {
    type Output = Money;
    
    fn mul(self, scalar: f64) -> Self::Output {
        let result_amount = self.amount() * scalar;
        Money::new(self.currency_code(), result_amount).unwrap()
    }
}

impl Div<f64> for Money {
    type Output = Money;
    
    fn div(self, scalar: f64) -> Self::Output {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        let result_amount = self.amount() / scalar;
        Money::new(self.currency_code(), result_amount).unwrap()
    }
}

// Display formatting
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let currency = self.currency();
        let amount = self.amount();
        let precision = currency.precision() as usize;
        
        if precision == 0 {
            write!(f, "{}{:.0}", currency.symbol(), amount)
        } else {
            write!(f, "{}{:.prec$}", currency.symbol(), amount, prec = precision)
        }
    }
}

// String parsing
impl FromStr for Money {
    type Err = CurrencyError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simple parsing: "USD:10.50" or "10.50 USD"
        if let Some((code, amount)) = s.split_once(':') {
            let value = amount.parse::<f64>()
                .map_err(|_| CurrencyError::invalid_amount(s.to_string(), ""))?;
            return Money::new(code.trim(), value);
        }
        
        // TODO: Implement more sophisticated parsing with currency symbols
        Err(CurrencyError::invalid_amount(s.to_string(), ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_creation() {
        let usd = Money::USD(10.50);
        assert_eq!(usd.currency_code(), "USD");
        assert_eq!(usd.amount(), 10.50);
        assert_eq!(usd.to_minor_units(), 1050);
    }

    #[test]
    fn test_money_new() {
        let usd = Money::new("USD", 10.50).unwrap();
        assert_eq!(usd.amount(), 10.50);
        
        let eur = Money::new("EUR", 25.75).unwrap();
        assert_eq!(eur.amount(), 25.75);
    }

    #[test]
    fn test_arithmetic() {
        let a = Money::USD(10.50);
        let b = Money::USD(5.25);
        let sum = (a + b).unwrap();
        assert_eq!(sum.amount(), 15.75);
        
        let diff = (Money::USD(30.00) - Money::USD(12.50)).unwrap();
        assert_eq!(diff.amount(), 17.50);
    }

    #[test]
    fn test_currency_mismatch() {
        let usd = Money::USD(10.0);
        let eur = Money::EUR(10.0);
        let result = usd + eur;
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let usd = Money::USD(10.50);
        assert_eq!(format!("{}", usd), "$10.50");
        
        let jpy = Money::JPY(1000.0);
        assert_eq!(format!("{}", jpy), "¥1000");
    }

    #[test]
    fn test_convenient_constructors() {
        let usd1 = Money::usd(10.50);
        let usd2 = Money::USD(10.50);
        assert_eq!(usd1, usd2);
        
        let btc1 = Money::btc(0.001);
        let btc2 = Money::BTC(0.001);
        assert_eq!(btc1, btc2);
    }

    #[test]
    fn test_rounding() {
        let usd = Money::USD(10.567);
        let rounded = usd.round_to_precision();
        assert_eq!(rounded.amount(), 10.57);
        
        let custom_rounded = usd.round(1);
        assert_eq!(custom_rounded.amount(), 10.6);
    }
}