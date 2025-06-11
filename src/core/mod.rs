use rust_decimal::Decimal;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;
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
// Impl for BigDecimal (high precision monetary calculations)
// =======================

impl Monetizable for BigDecimal {
    #[inline]
    fn zero() -> Self {
        BigDecimal::zero()
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.is_zero()
    }

    #[inline]
    fn try_from_f64(val: f64) -> Result<Self, MoneyError> {
        if val.is_finite() {
            // For better precision control, detect the number of decimal places needed
            let val_str = format!("{}", val);
            if let Ok(bd) = BigDecimal::from_str(&val_str) {
                Ok(bd)
            } else {
                // Fallback to the original method with scale 8
                Ok(BigDecimal::from_f64(val, 8))
            }
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f64(&self) -> Result<f64, MoneyError> {
        let result = self.to_f64();
        if result.is_finite() {
            Ok(result)
        } else {
            Err(MoneyError::ConversionError(format!("Cannot convert BigDecimal to f64: overflow or invalid value")))
        }
    }

    #[inline]
    fn try_from_f32(val: f32) -> Result<Self, MoneyError> {
        if val.is_finite() {
            // Use a default scale of 6 for f32 precision
            Ok(BigDecimal::from_f64(val as f64, 6))
        } else {
            Err(MoneyError::ConversionError(format!("Invalid float value: {}", val)))
        }
    }

    #[inline]
    fn try_to_f32(&self) -> Result<f32, MoneyError> {
        let f64_val = self.to_f64();
        if f64_val.is_finite() {
            let f32_val = f64_val as f32;
            if f32_val.is_finite() {
                Ok(f32_val)
            } else {
                Err(MoneyError::ConversionError(format!("Overflow converting BigDecimal to f32")))
            }
        } else {
            Err(MoneyError::ConversionError(format!("Cannot convert BigDecimal to f32: invalid value")))
        }
    }

    #[inline]
    fn try_from_decimal(val: Decimal) -> Result<Self, MoneyError> {
        // Convert Decimal to string and then parse as BigDecimal
        let decimal_str = val.to_string();
        println!("{} {}", val.scale(), val);
        BigDecimal::from_str(&decimal_str).map_err(|e| {
            MoneyError::ConversionError(format!("Cannot convert Decimal to BigDecimal: {}", e))
        })
    }

    #[inline]
    fn try_to_decimal(&self) -> Result<Decimal, MoneyError> {
        // Convert BigDecimal to string and then parse as Decimal
        let bigdecimal_str = self.to_string();
        Decimal::from_str(&bigdecimal_str).map_err(|e| {
            MoneyError::ConversionError(format!("Cannot convert BigDecimal to Decimal: {}", e))
        })
    }
}

// Arithmetic operations for BigDecimal
impl Add for BigDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        (&self).add(&other, &RoundingMode::HalfEven)
    }
}

impl Sub for BigDecimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.subtract(&other, &RoundingMode::HalfEven)
    }
}

impl Mul for BigDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let target_scale = (self.scale() + other.scale()).max(8); // Maintain reasonable precision
        self.multiply(&other, &RoundingMode::HalfEven, target_scale)
    }
}

impl Div for BigDecimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        self.divide(&other, &RoundingMode::HalfEven, 8).unwrap_or_else(|_| BigDecimal::zero())
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

    pub fn round_bigdecimal(&self, value: &BigDecimal) -> BigDecimal {
        value.with_scale(self.max_scale, &self.rounding_mode)
    }

    pub fn apply_precision<T: Monetizable + 'static>(&self, value: T) -> Result<T, MoneyError> {
        // For types that support precision application
        match std::any::TypeId::of::<T>() {
            id if id == std::any::TypeId::of::<BigDecimal>() => {
                // This is a bit hacky but works for demonstration
                let bd = unsafe { std::mem::transmute_copy::<T, BigDecimal>(&value) };
                let rounded = self.round_bigdecimal(&bd);
                Ok(unsafe { std::mem::transmute_copy::<BigDecimal, T>(&rounded) })
            }
            _ => Ok(value), // For other types, return as-is
        }
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
    pub fn new() -> Self {
        Self::default()
    }

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

    // Preset configurations
    pub fn high_precision() -> Self {
        Self {
            precision: Some(34),
            max_scale: Some(10),
            rounding_mode: Some(RoundingMode::HalfEven),
        }
    }

    pub fn currency_precision() -> Self {
        Self {
            precision: Some(19),
            max_scale: Some(2),
            rounding_mode: Some(RoundingMode::HalfEven),
        }
    }

    pub fn scientific_precision() -> Self {
        Self {
            precision: Some(50),
            max_scale: Some(15),
            rounding_mode: Some(RoundingMode::HalfEven),
        }
    }
}




// Enhanced Monetary struct
#[derive(Debug, Clone, PartialEq)]
pub struct Monetary<T: Monetizable  + 'static> {
    pub amount: T,
    pub currency: Currency,
    pub context: MonetaryContext,
}

impl<T: Monetizable + 'static> Monetary<T> {
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

    pub fn zero_with_context(currency: Currency, context: MonetaryContext) -> Self {
        Self::new_with_context(T::zero(), currency, context)
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

    pub fn context(&self) -> &MonetaryContext {
        &self.context
    }

    pub fn with_context(mut self, context: MonetaryContext) -> Self {
        self.context = context;
        self
    }

    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = currency;
        self
    }

    pub fn with_amount(mut self, amount: T) -> Self {
        self.amount = amount;
        self
    }

    // Apply context rounding to the amount
    pub fn apply_context(&self) -> Result<Self, MoneyError> {
        let rounded_amount = self.context.apply_precision(self.amount)?;
        Ok(Self::new_with_context(rounded_amount, self.currency.clone(), self.context.clone()))
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

    // Comparison methods
    pub fn is_positive(&self) -> bool {
        self.amount > T::zero()
    }

    pub fn is_negative(&self) -> bool {
        self.amount < T::zero()
    }

    // Absolute value
    pub fn abs(&self) -> Result<Self, MoneyError> where T: Default {
        let abs_amount = if self.is_negative() {
            T::zero() - self.amount
        } else {
            self.amount
        };
        Ok(Self::new_with_context(abs_amount, self.currency.clone(), self.context.clone()))
    }

    // Negate
    pub fn negate(&self) -> Self {
        Self::new_with_context(
            T::zero() - self.amount,
            self.currency.clone(),
            self.context.clone(),
        )
    }

    // Check if currencies are compatible for operations
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        self.currency == other.currency
    }

    // Safe arithmetic operations that check currency compatibility
    pub fn safe_add(&self, other: &Self) -> Result<Self, MoneyError> {
        if !self.is_compatible_with(other) {
            return Err(MoneyError::CurrencyMismatch(self.currency.clone(), other.currency.clone()));
        }
        Ok(Self::new_with_context(
            self.amount + other.amount,
            self.currency.clone(),
            self.context.clone(),
        ))
    }

    pub fn safe_subtract(&self, other: &Self) -> Result<Self, MoneyError> {
        if !self.is_compatible_with(other) {
            return Err(MoneyError::CurrencyMismatch(self.currency.clone(), other.currency.clone()));
        }
        Ok(Self::new_with_context(
            self.amount - other.amount,
            self.currency.clone(),
            self.context.clone(),
        ))
    }

    // Scalar operations
    pub fn multiply_by(&self, scalar: T) -> Self {
        Self::new_with_context(
            self.amount * scalar,
            self.currency.clone(),
            self.context.clone(),
        )
    }

    pub fn divide_by(&self, scalar: T) -> Self {
        Self::new_with_context(
            self.amount / scalar,
            self.currency.clone(),
            self.context.clone(),
        )
    }

    // Percentage operations
    pub fn apply_percentage(&self, percentage: f64) -> Result<Self, MoneyError> {
        let multiplier = T::try_from_f64(1.0 + percentage / 100.0)?;
        let result = self.multiply_by(multiplier);
        // Apply context rounding to the result
        result.apply_context()
    }

    pub fn percentage_of(&self, percentage: f64) -> Result<Self, MoneyError> {
        let multiplier = T::try_from_f64(percentage / 100.0)?;
        let result = self.multiply_by(multiplier);
        // Apply context rounding to the result
        result.apply_context()
    }
}

// Arithmetic operations for Monetary (same currency only)
impl<T: Monetizable> Add for Monetary<T> {
    type Output = Result<Self, MoneyError>;

    fn add(self, other: Self) -> Self::Output {
        self.safe_add(&other)
    }
}

impl<T: Monetizable> Sub for Monetary<T> {
    type Output = Result<Self, MoneyError>;

    fn sub(self, other: Self) -> Self::Output {
        self.safe_subtract(&other)
    }
}

// Scalar multiplication
impl<T: Monetizable> Mul<T> for Monetary<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        self.multiply_by(scalar)
    }
}

// Display implementation
impl<T: Monetizable> std::fmt::Display for Monetary<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}








#[cfg(test)]
mod tests {
    use super::*;
    // Convenience type aliases
    pub type DecimalMoney = Monetary<Decimal>;
    pub type FloatMoney = Monetary<f64>;
    // Convenience type alias
    pub type BigDecimalMoney = Monetary<BigDecimal>;


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




    // =====================
    // MonetaryContext Tests
    // =====================

    #[test]
    fn test_monetary_context_creation() {
        let context = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        assert_eq!(context.precision(), 19);
        assert_eq!(context.max_scale(), 2);
        assert_eq!(context.rounding_mode(), &RoundingMode::HalfEven);
    }

    #[test]
    fn test_monetary_context_default() {
        let context = MonetaryContext::default();
        assert_eq!(context.precision(), 19);
        assert_eq!(context.max_scale(), 6);
        assert_eq!(context.rounding_mode(), &RoundingMode::HalfEven);
    }

    #[test]
    fn test_monetary_context_rounding() {
        let context = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        
        // Test BigDecimal rounding
        let bd = BigDecimal::from_str("123.456").unwrap();
        let rounded = context.round_bigdecimal(&bd);
        assert_eq!(rounded.to_string(), "123.46");
        assert_eq!(rounded.scale(), 2);

        // Test with different rounding mode
        let context_up = MonetaryContext::new(19, 2, RoundingMode::Up);
        let rounded_up = context_up.round_bigdecimal(&bd);
        assert_eq!(rounded_up.to_string(), "123.46");
    }

    #[test]
    fn test_monetary_context_equality() {
        let context1 = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        let context2 = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        let context3 = MonetaryContext::new(19, 3, RoundingMode::HalfEven);

        assert_eq!(context1, context2);
        assert_ne!(context1, context3);
    }

    // ==========================
    // MonetaryContextBuilder Tests
    // ==========================

    #[test]
    fn test_monetary_context_builder_default() {
        let context = MonetaryContext::builder().build();
        assert_eq!(context.precision(), 19);
        assert_eq!(context.max_scale(), 6);
        assert_eq!(context.rounding_mode(), &RoundingMode::HalfEven);
    }

    #[test]
    fn test_monetary_context_builder_custom() {
        let context = MonetaryContext::builder()
            .with_precision(34)
            .with_max_scale(4)
            .with_rounding_mode(RoundingMode::Ceiling)
            .build();

        assert_eq!(context.precision(), 34);
        assert_eq!(context.max_scale(), 4);
        assert_eq!(context.rounding_mode(), &RoundingMode::Ceiling);
    }

    #[test]
    fn test_monetary_context_builder_partial() {
        let context = MonetaryContext::builder()
            .with_precision(25)
            .build();

        assert_eq!(context.precision(), 25);
        assert_eq!(context.max_scale(), 6); // Default
        assert_eq!(context.rounding_mode(), &RoundingMode::HalfEven); // Default
    }

    #[test]
    fn test_monetary_context_builder_presets() {
        // Test high precision preset
        let high_precision = MonetaryContextBuilder::high_precision().build();
        assert_eq!(high_precision.precision(), 34);
        assert_eq!(high_precision.max_scale(), 10);
        assert_eq!(high_precision.rounding_mode(), &RoundingMode::HalfEven);

        // Test currency precision preset
        let currency_precision = MonetaryContextBuilder::currency_precision().build();
        assert_eq!(currency_precision.precision(), 19);
        assert_eq!(currency_precision.max_scale(), 2);
        assert_eq!(currency_precision.rounding_mode(), &RoundingMode::HalfEven);

        // Test scientific precision preset
        let scientific_precision = MonetaryContextBuilder::scientific_precision().build();
        assert_eq!(scientific_precision.precision(), 50);
        assert_eq!(scientific_precision.max_scale(), 15);
        assert_eq!(scientific_precision.rounding_mode(), &RoundingMode::HalfEven);
    }

    #[test]
    fn test_monetary_context_builder_chaining() {
        let context = MonetaryContextBuilder::new()
            .with_precision(20)
            .with_max_scale(3)
            .with_rounding_mode(RoundingMode::Floor)
            .build();

        assert_eq!(context.precision(), 20);
        assert_eq!(context.max_scale(), 3);
        assert_eq!(context.rounding_mode(), &RoundingMode::Floor);
    }

    // =====================
    // Enhanced Monetary Tests
    // =====================

    #[test]
    fn test_monetary_with_context() {
        let context = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        let money = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("100.50").unwrap(),
            Currency::usd(),
            context.clone()
        );

        assert_eq!(money.context(), &context);
        assert_eq!(money.amount().to_string(), "100.50");
        assert_eq!(money.currency().symbol(), Currency::usd().symbol());
    }

    #[test]
    fn test_monetary_zero_with_context() {
        let context = MonetaryContext::new(19, 4, RoundingMode::Ceiling);
        let zero_money = BigDecimalMoney::zero_with_context(Currency::eur(), context.clone());

        assert!(zero_money.is_zero());
        assert_eq!(zero_money.context(), &context);
        assert_eq!(zero_money.currency(), &Currency::eur());
    }

    #[test]
    fn test_monetary_builder_pattern() {
        let context = MonetaryContext::new(19, 2, RoundingMode::HalfEven);
        let original_money = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );

        let modified_money = original_money
            .with_context(context.clone())
            .with_currency(Currency::eur())
            .with_amount(BigDecimal::from_str("85.00").unwrap());

        assert_eq!(modified_money.context(), &context);
        assert_eq!(modified_money.currency(), &Currency::eur());
        assert_eq!(modified_money.amount().to_string(), "85.00");
    }

    #[test]
    fn test_monetary_sign_checks() {
        let positive = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );
        let negative = BigDecimalMoney::new(
            BigDecimal::from_str("-50.00").unwrap(),
            Currency::usd()
        );
        let zero = BigDecimalMoney::zero(Currency::usd());

        assert!(positive.is_positive());
        assert!(!positive.is_negative());
        assert!(!positive.is_zero());

        assert!(!negative.is_positive());
        assert!(negative.is_negative());
        assert!(!negative.is_zero());

        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
        assert!(zero.is_zero());
    }

    #[test]
    fn test_monetary_abs_and_negate() {
        let positive = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );
        let negative = BigDecimalMoney::new(
            BigDecimal::from_str("-50.00").unwrap(),
            Currency::usd()
        );

        // Test absolute value
        let abs_positive = positive.abs().unwrap();
        let abs_negative = negative.abs().unwrap();
        assert_eq!(abs_positive.amount().to_string(), "100.00");
        assert_eq!(abs_negative.amount().to_string(), "50.00");

        // Test negate
        let neg_positive = positive.negate();
        let neg_negative = negative.negate();
        assert_eq!(neg_positive.amount().to_string(), "-100.00");
        assert_eq!(neg_negative.amount().to_string(), "50.00");
    }

    #[test]
    fn test_monetary_compatibility_check() {
        let usd_money = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );
        let eur_money = BigDecimalMoney::new(
            BigDecimal::from_str("85.00").unwrap(),
            Currency::eur()
        );
        let other_usd_money = BigDecimalMoney::new(
            BigDecimal::from_str("50.00").unwrap(),
            Currency::usd()
        );

        assert!(usd_money.is_compatible_with(&other_usd_money));
        assert!(!usd_money.is_compatible_with(&eur_money));
    }

    #[test]
    fn test_monetary_safe_arithmetic() {
        let money1 = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );
        let money2 = BigDecimalMoney::new(
            BigDecimal::from_str("50.00").unwrap(),
            Currency::usd()
        );
        let eur_money = BigDecimalMoney::new(
            BigDecimal::from_str("85.00").unwrap(),
            Currency::eur()
        );

        // Test safe addition
        let sum = money1.safe_add(&money2).unwrap();
        assert_eq!(sum.amount().to_string(), "150.00");
        assert_eq!(sum.currency(), &Currency::usd());

        // Test safe subtraction
        let diff = money1.safe_subtract(&money2).unwrap();
        assert_eq!(diff.amount().to_string(), "50.00");

        // Test currency mismatch errors
        assert!(money1.safe_add(&eur_money).is_err());
        assert!(money1.safe_subtract(&eur_money).is_err());
    }

    #[test]
    fn test_monetary_scalar_operations() {
        let money = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );
        println!("{}", money);
        let scalar = BigDecimal::from_str("2.5").unwrap();
        println!("{}", scalar);

        // Test multiplication
        let product = money.multiply_by(scalar);
        assert_eq!(product.amount().to_string(), "250.00000000");
        assert_eq!(product.currency(), &Currency::usd());

        // Test division
        let quotient = money.divide_by(BigDecimal::from_str("4.0").unwrap());
        assert_eq!(quotient.amount().to_string(), "25.00000000");
    }

    #[test]
    fn test_monetary_percentage_operations() {
        let money = BigDecimalMoney::new(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd()
        );

        println!("{} {}", money.amount.scale(), money.amount.unscaled_value());
        // Test applying percentage (increase by 20%)
        let increased = money.apply_percentage(20.0).unwrap();
        println!("{} {}", increased.amount.scale(), increased.amount.unscaled_value());
        assert_eq!(increased.amount().to_string(), "120.0000000000");

        // Test taking percentage (20% of amount)
        let percentage = money.percentage_of(20.0).unwrap();
        assert_eq!(percentage.amount().to_string(), "20.0000000000");

        // Test negative percentage (decrease by 10%)
        let decreased = money.apply_percentage(-10.0).unwrap();
        assert_eq!(decreased.amount().to_string(), "90.0000000000");
    }


    // In src/core/mod.rs, inside the #[cfg(test)] mod tests { ... } block
    #[test]
    fn test_monetary_percentage_operations_with_context() {
        // Context with specific precision (2 decimal places for currency) and rounding mode
        let context = MonetaryContextBuilder::currency_precision()
            .with_rounding_mode(RoundingMode::HalfUp)
            .build();

        let initial_money = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("100.555").unwrap(), // Amount that requires rounding
            Currency::usd(),
            context.clone()
        );

        // Test applying percentage (increase by 10%)
        // 100.555 * 1.10 = 110.6105
        // With context (max_scale 2, HalfUp): 110.61
        let increased = initial_money.apply_percentage(10.0).unwrap();
        assert_eq!(increased.amount().to_string(), "110.61");
        assert_eq!(increased.currency(), &Currency::usd());
        assert_eq!(increased.context(), &context);

        // Test taking percentage (20% of amount)
        // 100.555 * 0.20 = 20.111
        // With context (max_scale 2, HalfUp): 20.11
        let percentage_of_amount = initial_money.percentage_of(20.0).unwrap();
        assert_eq!(percentage_of_amount.amount().to_string(), "20.11");
        assert_eq!(percentage_of_amount.currency(), &Currency::usd());
        assert_eq!(percentage_of_amount.context(), &context);

        // Test with negative percentage (decrease by 5%, rounds to 2 decimal places)
        // 100.555 * (1 - 0.05) = 100.555 * 0.95 = 95.52725
        // With context (max_scale 2, HalfUp): 95.53
        let decreased = initial_money.apply_percentage(-5.0).unwrap();
        assert_eq!(decreased.amount().to_string(), "95.53");
        assert_eq!(decreased.currency(), &Currency::usd());
        assert_eq!(decreased.context(), &context);

        // Test a value that rounds differently with HalfUp
        let another_money = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("100.005").unwrap(),
            Currency::usd(),
            context.clone()
        );
        // 100.005 * 1.0 = 100.005
        // With context (max_scale 2, HalfUp): 100.01
        let no_change_rounded = another_money.apply_percentage(0.0).unwrap();
        assert_eq!(no_change_rounded.amount().to_string(), "100.01");
    }

    #[test]
    fn test_monetary_context_conversion_with_precision() {
        let high_precision_context = MonetaryContextBuilder::high_precision().build();
        let currency_context = MonetaryContextBuilder::currency_precision().build();

        let money = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("100.123456789").unwrap(),
            Currency::usd(),
            high_precision_context
        );

        // Convert to currency precision context
        let currency_money = money.with_context(currency_context.clone());
        assert_eq!(currency_money.context(), &currency_context);
        
        // Apply context rounding
        let rounded_money = currency_money.apply_context().unwrap();
        // Note: This test assumes the apply_context method works correctly
        assert_eq!(rounded_money.context(), &currency_context);
    }

    #[test]
    fn test_monetary_conversion_with_context_preservation() {
        let context = MonetaryContext::new(19, 4, RoundingMode::Ceiling);
        let usd_money = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("100.00").unwrap(),
            Currency::usd(),
            context.clone()
        );

        // Test currency conversion preserves context
        let eur_money = usd_money.convert::<BigDecimal>(0.85, Currency::eur()).unwrap();
        assert_eq!(eur_money.context(), &context);
        assert_eq!(eur_money.currency(), &Currency::eur());

        // Test type conversion preserves context
        let f64_money = usd_money.as_type::<f64>().unwrap();
        assert_eq!(f64_money.context(), &context);
        assert_eq!(f64_money.currency(), &Currency::usd());
    }

    #[test]
    fn test_monetary_complex_operations_with_context() {
        let context = MonetaryContextBuilder::currency_precision()
            .with_rounding_mode(RoundingMode::HalfUp)
            .build();

        let principal = BigDecimalMoney::new_with_context(
            BigDecimal::from_str("1000.00").unwrap(),
            Currency::usd(),
            context.clone()
        );

        // Calculate compound interest: A = P(1 + r)^t
        let annual_rate = 0.05; // 5%
        let years = 3;

        let mut amount = principal;
        for _ in 0..years {
            amount = amount.apply_percentage(annual_rate * 100.0).unwrap();
        }

        // Verify context is preserved through operations
        assert_eq!(amount.context(), &context);
        assert_eq!(amount.currency(), &Currency::usd());
        
        // 1000 * 1.05^3 = 1157.625, but depends on BigDecimal precision
        let final_amount = amount.amount().try_to_f64().unwrap();
        assert!((final_amount - 1157.625).abs() < 0.01);
    }

}