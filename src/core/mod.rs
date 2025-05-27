use rust_decimal::Decimal;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

pub mod currency;
pub mod currency_unit;
pub mod types;
pub mod money;

use crate::core::currency::Currency;
use crate::core::currency_unit::CurrencyUnit;
use crate::constants::RoundingMode;
use crate::core::types::BigDecimal;
use rust_decimal::prelude::ToPrimitive;





// Custom error types for better error handling
#[derive(Debug, Clone, PartialEq)]
pub enum MoneyError {
    ConversionError(String),
    CurrencyMismatch(Currency, Currency),
    InvalidExchangeRate(f64),
    PrecisionLoss,
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            MoneyError::CurrencyMismatch(c1, c2) => write!(f, "Currency mismatch: {:?} vs {:?}", c1, c2),
            MoneyError::InvalidExchangeRate(rate) => write!(f, "Invalid exchange rate: {}", rate),
            MoneyError::PrecisionLoss => write!(f, "Precision loss in conversion"),
        }
    }
}

impl std::error::Error for MoneyError {}

/// Trait to abstract money-compatible numeric types, supporting common conversions and arithmetic.
pub trait Monetizable:
    Copy
    + Clone
    + Default
    + PartialEq
    + PartialOrd
    + std::fmt::Debug
    + std::fmt::Display
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
    fn zero() -> Self;
    fn is_zero(&self) -> bool;

    // Safe conversion methods that return Results
    fn try_from_f64(val: f64) -> Result<Self, MoneyError>;
    fn try_to_f64(&self) -> Result<f64, MoneyError>;

    fn try_from_f32(val: f32) -> Result<Self, MoneyError>;
    fn try_to_f32(&self) -> Result<f32, MoneyError>;

    fn try_from_decimal(val: Decimal) -> Result<Self, MoneyError>;
    fn try_to_decimal(&self) -> Result<Decimal, MoneyError>;

    // Convenience methods for backwards compatibility (deprecated)
    #[deprecated(note = "Use try_from_* methods instead")]
    fn from_f64(val: f64) -> Self {
        Self::try_from_f64(val).unwrap_or_default()
    }
    
    #[deprecated(note = "Use try_to_* methods instead")]
    fn to_f64(&self) -> Option<f64> {
        self.try_to_f64().ok()
    }
}

// =======================
// Impl for Decimal (recommended for monetary calculations)
// =======================

impl Monetizable for Decimal {
    #[inline]
    fn zero() -> Self {
        Decimal::ZERO
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == Decimal::ZERO
    }

    #[inline]
    fn try_from_f64(val: f64) -> Result<Self, MoneyError> {
        if val.is_finite() {
            Decimal::try_from(val).map_err(|_| MoneyError::ConversionError(format!("Cannot convert {} to Decimal", val)))
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f64(&self) -> Result<f64, MoneyError> {
        ToPrimitive::to_f64(self).ok_or(MoneyError::PrecisionLoss)

    }

    #[inline]
    fn try_from_f32(val: f32) -> Result<Self, MoneyError> {
        if val.is_finite() {
            Decimal::try_from(val).map_err(|_| MoneyError::ConversionError(format!("Cannot convert {} to Decimal", val)))
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f32(&self) -> Result<f32, MoneyError> {
        self.to_f32().ok_or(MoneyError::PrecisionLoss)
    }

    #[inline]
    fn try_from_decimal(val: Decimal) -> Result<Self, MoneyError> {
        Ok(val)
    }

    #[inline]
    fn try_to_decimal(&self) -> Result<Decimal, MoneyError> {
        Ok(*self)
    }
}

// =======================
// Impl for f64 (not recommended for monetary calculations)
// =======================

impl Monetizable for f64 {
    #[inline]
    fn zero() -> Self {
        0.0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        *self == 0.0
    }

    #[inline]
    fn try_from_f64(val: f64) -> Result<Self, MoneyError> {
        if val.is_finite() {
            Ok(val)
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f64(&self) -> Result<f64, MoneyError> {
        if self.is_finite() {
            Ok(*self)
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", self)))
        }
    }

    #[inline]
    fn try_from_f32(val: f32) -> Result<Self, MoneyError> {
        if val.is_finite() {
            Ok(val as f64)
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f32(&self) -> Result<f32, MoneyError> {
        if self.is_finite() {
            Ok(*self as f32)
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", self)))
        }
    }

    #[inline]
    fn try_from_decimal(val: Decimal) -> Result<Self, MoneyError> {
        val.try_to_f64()
    }

    #[inline]
    fn try_to_decimal(&self) -> Result<Decimal, MoneyError> {
        Decimal::try_from_f64(*self)
    }
}











// =======================
// Enhanced MonetaryContext
// =======================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonetaryContext {
    precision: u32,
    max_scale: i32,
    rounding_mode: RoundingMode,
}

impl MonetaryContext {
    pub fn new(precision: u32, max_scale: i32, rounding_mode: RoundingMode) -> Self {
        Self {
            precision,
            max_scale,
            rounding_mode,
        }
    }

    pub fn builder() -> MonetaryContextBuilder {
        MonetaryContextBuilder::default()
    }

    pub fn precision(&self) -> u32 {
        self.precision
    }

    pub fn max_scale(&self) -> i32 {
        self.max_scale
    }

    pub fn rounding_mode(&self) -> &RoundingMode {
        &self.rounding_mode
    }

    pub fn round_decimal(&self, value: Decimal) -> Decimal {
        value.round_dp(self.max_scale as u32)
    }
}

impl Default for MonetaryContext {
    fn default() -> Self {
        Self {
            precision: 19,
            max_scale: 6,
            rounding_mode: RoundingMode::HalfEven,
        }
    }
}

#[derive(Debug, Default)]
pub struct MonetaryContextBuilder {
    precision: Option<u32>,
    max_scale: Option<i32>,
    rounding_mode: Option<RoundingMode>,
}

impl MonetaryContextBuilder {
    pub fn with_precision(mut self, precision: u32) -> Self {
        self.precision = Some(precision);
        self
    }

    pub fn with_max_scale(mut self, max_scale: i32) -> Self {
        self.max_scale = Some(max_scale);
        self
    }

    pub fn with_rounding_mode(mut self, rounding_mode: RoundingMode) -> Self {
        self.rounding_mode = Some(rounding_mode);
        self
    }

    pub fn build(self) -> MonetaryContext {
        MonetaryContext {
            precision: self.precision.unwrap_or(19),
            max_scale: self.max_scale.unwrap_or(6),
            rounding_mode: self.rounding_mode.unwrap_or(RoundingMode::HalfEven),
        }
    }
}


// =======================
// Enhanced Monetary struct
// =======================

#[derive(Debug, Clone, PartialEq)]
pub struct Monetary<T: Monetizable> {
    pub amount: T,
    pub currency: Currency,
    pub context: MonetaryContext,
}

impl<T: Monetizable> Monetary<T> {
    pub fn new(amount: T, currency: Currency) -> Self {
        Self { 
            amount, 
            currency,
            context: MonetaryContext::default(),
        }
    }

    pub fn new_with_context(amount: T, currency: Currency, context: MonetaryContext) -> Self {
        Self { amount, currency, context }
    }

    pub fn zero(currency: Currency) -> Self {
        Self::new(T::zero(), currency)
    }

    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn amount(&self) -> &T {
        &self.amount
    }

    // Safe currency conversion
    pub fn convert<U: Monetizable>(&self, rate: f64, target_currency: Currency) -> Result<Monetary<U>, MoneyError> {
        if rate <= 0.0 {
            return Err(MoneyError::InvalidExchangeRate(rate));
        }

        let current_f64 = self.amount.try_to_f64()?;
        let new_amount_f64 = current_f64 * rate;
        let new_amount = U::try_from_f64(new_amount_f64)?;
        
        Ok(Monetary::new_with_context(new_amount, target_currency, self.context.clone()))
    }

    // Convert to different numeric type (same currency)
    pub fn as_type<U: Monetizable>(&self) -> Result<Monetary<U>, MoneyError> {
        let decimal_amount = self.amount.try_to_decimal()?;
        let new_amount = U::try_from_decimal(decimal_amount)?;
        Ok(Monetary::new_with_context(new_amount, self.currency.clone(), self.context.clone()))
    }
}

// Arithmetic operations for Monetary (same currency only)
impl<T: Monetizable> Add for Monetary<T> {
    type Output = Result<Self, MoneyError>;

    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Monetary::new_with_context(
            self.amount + other.amount,
            self.currency,
            self.context,
        ))
    }
}

impl<T: Monetizable> Sub for Monetary<T> {
    type Output = Result<Self, MoneyError>;

    fn sub(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Monetary::new_with_context(
            self.amount - other.amount,
            self.currency,
            self.context,
        ))
    }
}

// Scalar multiplication
impl<T: Monetizable> Mul<T> for Monetary<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Monetary::new_with_context(
            self.amount * scalar,
            self.currency,
            self.context,
        )
    }
}

// Display implementation
impl<T: Monetizable> fmt::Display for Monetary<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?}", self.amount, self.currency)
    }
}

// Convenience type aliases
pub type DecimalMoney = Monetary<Decimal>;
pub type FloatMoney = Monetary<f64>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decimal_money_creation() {
        let money = DecimalMoney::new(Decimal::new(100, 2), Currency::usd());
        assert_eq!(money.amount, Decimal::new(100, 2));
        assert_eq!(money.currency, Currency::usd());
    }

    #[test]
    fn test_currency_mismatch_error() {
        let usd = DecimalMoney::new(Decimal::new(100, 2), Currency::usd());
        let eur = DecimalMoney::new(Decimal::new(85, 2), Currency::eur());
        
        let result = usd + eur;
        assert!(matches!(result, Err(MoneyError::CurrencyMismatch(_, _))));
    }

    #[test]
    fn test_safe_conversion() {
        let decimal_money = DecimalMoney::new(Decimal::new(100, 2), Currency::usd());
        let float_result = decimal_money.convert::<f64>(1.2, Currency::eur());
        assert!(float_result.is_ok());
    }
}