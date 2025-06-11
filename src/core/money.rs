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
    NZD(f64),  // New Zealand dollars
    HKD(f64),  // Hong Kong dollars
    THB(f64),  // baht
    PHP(f64),  // Philippine pesos
    MYR(f64),  // Malaysian ringgit
    IDR(f64),  // Indonesian rupiah
    EGP(f64),  // Egyptian pounds
    CLP(f64),  // Chilean pesos

    // European Currencies
    NOK(f64),  // kroner
    SEK(f64),  // kronor
    DKK(f64),  // kroner
    PLN(f64),  // zloty
    CZK(f64),  // koruny
    HUF(f64),  // forint
    ISK(f64),  // Icelandic króna
    RON(f64),  // Romanian leu
    HRK(f64),  // Croatian kuna (Note: Croatia adopted EUR in 2023, but keeping for historical context or if needed)

    // Middle East / Africa
    ILS(f64),  // shekels
    AED(f64),  // dirhams
    SAR(f64),  // riyals
    TRY(f64),  // lira
    KWD(f64),  // Kuwaiti dinars
    QAR(f64),  // Qatari riyals
    MAD(f64),  // Moroccan dirhams
    NGN(f64),  // Nigerian naira

    // Cryptocurrencies
    BTC(f64),  // bitcoins
    ETH(f64),  // ether
    LTC(f64),  // litecoins
    XRP(f64),  // ripple
    ADA(f64),  // cardano
    DOGE(f64), // dogecoin
    DOT(f64),  // polkadot
    SOL(f64),  // solana
    USDT(f64), // tether (stablecoin)
    USDC(f64), // USD Coin (stablecoin)

    // Precious Metals
    XAU(f64),  // troy ounces of gold
    XAG(f64),  // troy ounces of silver
    XPT(f64),  // troy ounces of platinum
    XPD(f64),  // troy ounces of palladium
    XRH(f64),  // troy ounces of rhodium
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
            "NZD" => Ok(Money::NZD(amount)),
            "HKD" => Ok(Money::HKD(amount)),
            "THB" => Ok(Money::THB(amount)),
            "PHP" => Ok(Money::PHP(amount)),
            "MYR" => Ok(Money::MYR(amount)),
            "IDR" => Ok(Money::IDR(amount)),
            "EGP" => Ok(Money::EGP(amount)),
            "CLP" => Ok(Money::CLP(amount)),
            "NOK" => Ok(Money::NOK(amount)),
            "SEK" => Ok(Money::SEK(amount)),
            "DKK" => Ok(Money::DKK(amount)),
            "PLN" => Ok(Money::PLN(amount)),
            "CZK" => Ok(Money::CZK(amount)),
            "HUF" => Ok(Money::HUF(amount)),
            "ISK" => Ok(Money::ISK(amount)),
            "RON" => Ok(Money::RON(amount)),
            "HRK" => Ok(Money::HRK(amount)),
            "ILS" => Ok(Money::ILS(amount)),
            "AED" => Ok(Money::AED(amount)),
            "SAR" => Ok(Money::SAR(amount)),
            "TRY" => Ok(Money::TRY(amount)),
            "KWD" => Ok(Money::KWD(amount)),
            "QAR" => Ok(Money::QAR(amount)),
            "MAD" => Ok(Money::MAD(amount)),
            "NGN" => Ok(Money::NGN(amount)),
            "BTC" => Ok(Money::BTC(amount)),
            "ETH" => Ok(Money::ETH(amount)),
            "LTC" => Ok(Money::LTC(amount)),
            "XRP" => Ok(Money::XRP(amount)),
            "ADA" => Ok(Money::ADA(amount)),
            "DOGE" => Ok(Money::DOGE(amount)),
            "DOT" => Ok(Money::DOT(amount)),
            "SOL" => Ok(Money::SOL(amount)),
            "USDT" => Ok(Money::USDT(amount)),
            "USDC" => Ok(Money::USDC(amount)),
            "XAU" => Ok(Money::XAU(amount)),
            "XAG" => Ok(Money::XAG(amount)),
            "XPT" => Ok(Money::XPT(amount)),
            "XPD" => Ok(Money::XPD(amount)),
            "XRH" => Ok(Money::XRH(amount)),
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
            Money::NZD(_) => "NZD",
            Money::HKD(_) => "HKD",
            Money::THB(_) => "THB",
            Money::PHP(_) => "PHP",
            Money::MYR(_) => "MYR",
            Money::IDR(_) => "IDR",
            Money::EGP(_) => "EGP",
            Money::CLP(_) => "CLP",
            Money::NOK(_) => "NOK",
            Money::SEK(_) => "SEK",
            Money::DKK(_) => "DKK",
            Money::PLN(_) => "PLN",
            Money::CZK(_) => "CZK",
            Money::HUF(_) => "HUF",
            Money::ISK(_) => "ISK",
            Money::RON(_) => "RON",
            Money::HRK(_) => "HRK",
            Money::ILS(_) => "ILS",
            Money::AED(_) => "AED",
            Money::SAR(_) => "SAR",
            Money::TRY(_) => "TRY",
            Money::KWD(_) => "KWD",
            Money::QAR(_) => "QAR",
            Money::MAD(_) => "MAD",
            Money::NGN(_) => "NGN",
            Money::BTC(_) => "BTC",
            Money::ETH(_) => "ETH",
            Money::LTC(_) => "LTC",
            Money::XRP(_) => "XRP",
            Money::ADA(_) => "ADA",
            Money::DOGE(_) => "DOGE",
            Money::DOT(_) => "DOT",
            Money::SOL(_) => "SOL",
            Money::USDT(_) => "USDT",
            Money::USDC(_) => "USDC",
            Money::XAU(_) => "XAU",
            Money::XAG(_) => "XAG",
            Money::XPT(_) => "XPT",
            Money::XPD(_) => "XPD",
            Money::XRH(_) => "XRH",
        }
    }

    /// Get the raw amount value
    pub fn amount(&self) -> f64 {
        match self {
            Money::USD(v) | Money::EUR(v) | Money::GBP(v) | Money::JPY(v) |
            Money::CHF(v) | Money::CAD(v) | Money::AUD(v) | Money::CNY(v) |
            Money::INR(v) | Money::KRW(v) | Money::BRL(v) | Money::RUB(v) |
            Money::ZAR(v) | Money::MXN(v) | Money::SGD(v) | Money::NZD(v) |
            Money::HKD(v) | Money::THB(v) | Money::PHP(v) | Money::MYR(v) |
            Money::IDR(v) | Money::EGP(v) | Money::CLP(v) | Money::NOK(v) |
            Money::SEK(v) | Money::DKK(v) | Money::PLN(v) | Money::CZK(v) |
            Money::HUF(v) | Money::ISK(v) | Money::RON(v) | Money::HRK(v) |
            Money::ILS(v) | Money::AED(v) | Money::SAR(v) | Money::TRY(v) |
            Money::KWD(v) | Money::QAR(v) | Money::MAD(v) | Money::NGN(v) |
            Money::BTC(v) | Money::ETH(v) | Money::LTC(v) | Money::XRP(v) |
            Money::ADA(v) | Money::DOGE(v) | Money::DOT(v) | Money::SOL(v) |
            Money::USDT(v) | Money::USDC(v) | Money::XAU(v) | Money::XAG(v) |
            Money::XPT(v) | Money::XPD(v) | Money::XRH(v) => *v,
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
    pub fn inr(rupees: f64) -> Self { Money::INR(rupees) }
    pub fn krw(won: f64) -> Self { Money::KRW(won) }
    pub fn brl(reais: f64) -> Self { Money::BRL(reais) }
    pub fn rub(rubles: f64) -> Self { Money::RUB(rubles) }
    pub fn zar(rand: f64) -> Self { Money::ZAR(rand) }
    pub fn mxn(pesos: f64) -> Self { Money::MXN(pesos) }
    pub fn sgd(dollars: f64) -> Self { Money::SGD(dollars) }
    pub fn nzd(dollars: f64) -> Self { Money::NZD(dollars) }
    pub fn hkd(dollars: f64) -> Self { Money::HKD(dollars) }
    pub fn thb(baht: f64) -> Self { Money::THB(baht) }
    pub fn php(pesos: f64) -> Self { Money::PHP(pesos) }
    pub fn myr(ringgit: f64) -> Self { Money::MYR(ringgit) }
    pub fn idr(rupiah: f64) -> Self { Money::IDR(rupiah) }
    pub fn egp(pounds: f64) -> Self { Money::EGP(pounds) }
    pub fn clp(pesos: f64) -> Self { Money::CLP(pesos) }
    pub fn nok(kroner: f64) -> Self { Money::NOK(kroner) }
    pub fn sek(kronor: f64) -> Self { Money::SEK(kronor) }
    pub fn dkk(kroner: f64) -> Self { Money::DKK(kroner) }
    pub fn pln(zloty: f64) -> Self { Money::PLN(zloty) }
    pub fn czk(koruny: f64) -> Self { Money::CZK(koruny) }
    pub fn huf(forint: f64) -> Self { Money::HUF(forint) }
    pub fn isk(krona: f64) -> Self { Money::ISK(krona) }
    pub fn ron(leu: f64) -> Self { Money::RON(leu) }
    pub fn hrk(kuna: f64) -> Self { Money::HRK(kuna) }
    pub fn ils(shekels: f64) -> Self { Money::ILS(shekels) }
    pub fn aed(dirhams: f64) -> Self { Money::AED(dirhams) }
    pub fn sar(riyals: f64) -> Self { Money::SAR(riyals) }
    pub fn r#try(lira: f64) -> Self { Money::TRY(lira) } // 'try' is a Rust keyword, so we use r#try
    pub fn kwd(dinars: f64) -> Self { Money::KWD(dinars) }
    pub fn qar(riyals: f64) -> Self { Money::QAR(riyals) }
    pub fn mad(dirhams: f64) -> Self { Money::MAD(dirhams) }
    pub fn ngn(naira: f64) -> Self { Money::NGN(naira) }
    pub fn btc(bitcoins: f64) -> Self { Money::BTC(bitcoins) }
    pub fn eth(ether: f64) -> Self { Money::ETH(ether) }
    pub fn ltc(litecoins: f64) -> Self { Money::LTC(litecoins) }
    pub fn xrp(ripple: f64) -> Self { Money::XRP(ripple) }
    pub fn ada(cardano: f64) -> Self { Money::ADA(cardano) }
    pub fn doge(dogecoin: f64) -> Self { Money::DOGE(dogecoin) }
    pub fn dot(polkadot: f64) -> Self { Money::DOT(polkadot) }
    pub fn sol(solana: f64) -> Self { Money::SOL(solana) }
    pub fn usdt(tether: f64) -> Self { Money::USDT(tether) }
    pub fn usdc(usd_coin: f64) -> Self { Money::USDC(usd_coin) }
    pub fn xau(troy_ounces: f64) -> Self { Money::XAU(troy_ounces) }
    pub fn xag(troy_ounces: f64) -> Self { Money::XAG(troy_ounces) }
    pub fn xpt(troy_ounces: f64) -> Self { Money::XPT(troy_ounces) }
    pub fn xpd(troy_ounces: f64) -> Self { Money::XPD(troy_ounces) }
    pub fn xrh(troy_ounces: f64) -> Self { Money::XRH(troy_ounces) }
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