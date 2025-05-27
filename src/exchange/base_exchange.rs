use crate::core::{Monetary, Monetizable, MonetaryContext};
use crate::core::currency::Currency;
use crate::core::currency_unit::CurrencyUnit;
use crate::constants::RoundingMode;
use crate::errors::ExchangeError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use rust_decimal::Decimal;

/// Fast hash-based key for currency pairs using numeric codes
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyPair {
    base_code: i32,
    target_code: i32,
}

impl CurrencyPair {
    pub fn new(base: &Currency, target: &Currency) -> Self {
        Self {
            base_code: base.numeric_code(),
            target_code: target.numeric_code(),
        }
    }
    
    pub fn from_units(base: &CurrencyUnit, target: &CurrencyUnit) -> Self {
        Self {
            base_code: base.get_numeric_code(),
            target_code: target.get_numeric_code(),
        }
    }
}

/// High-performance exchange rate with monetizable factor
#[derive(Debug, Clone)]
pub struct ExchangeRate<T: Monetizable> {
    base_currency: Currency,
    target_currency: Currency,
    factor: T,
    timestamp: Instant,
    ttl: Option<Duration>,
    context: MonetaryContext,
}

impl<T: Monetizable> ExchangeRate<T> {
    pub fn new(
        base_currency: Currency, 
        target_currency: Currency, 
        factor: T
    ) -> Self {
        Self {
            base_currency,
            target_currency,
            factor,
            timestamp: Instant::now(),
            ttl: None,
            context: MonetaryContext::default(),
        }
    }
    
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.ttl = Some(ttl);
        self
    }
    
    pub fn with_context(mut self, context: MonetaryContext) -> Self {
        self.context = context;
        self
    }

    pub fn get_base_currency(&self) -> &Currency {
        &self.base_currency
    }


    pub fn get_ttl(&self) -> &Option<Duration> {
        &self.ttl
    }

    pub fn get_target_currency(&self) -> &Currency {
        &self.target_currency
    }

    pub fn get_factor(&self) -> &T {
        &self.factor
    }
    
    pub fn get_context(&self) -> &MonetaryContext {
        &self.context
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            self.timestamp.elapsed() > ttl
        } else {
            false
        }
    }

    /// Fast application with same numeric type
    pub fn apply(&self, amount: &Monetary<T>) -> Result<Monetary<T>, ExchangeError> {
        if amount.currency != self.base_currency {
            return Err(ExchangeError::CurrencyMismatch);
        }
        
        if self.is_expired() {
            return Err(ExchangeError::ExpiredRate);
        }

        // Direct multiplication using Monetizable trait
        let converted_amount = amount.amount * self.factor;
        
        Ok(Monetary::new(converted_amount, self.target_currency.clone()))
    }
    
/// Cross-type conversion with rounding
pub fn apply_convert<U: Monetizable>(&self, amount: &Monetary<T>) -> Result<Monetary<U>, ExchangeError> {
    if amount.currency != self.base_currency {
        return Err(ExchangeError::CurrencyMismatch);
    }

    if self.is_expired() {
        return Err(ExchangeError::ExpiredRate);
    }

    // Convert to common decimal representation for precise calculation
    let amount_decimal = amount.amount.try_to_decimal()
        .map_err(|_| ExchangeError::ConversionError)?;
    let factor_decimal = self.factor.try_to_decimal()
        .map_err(|_|ExchangeError::ConversionError)?;

    let result_decimal = amount_decimal * factor_decimal;

    // Apply rounding based on context
    let rounded_decimal = self.apply_rounding(result_decimal);

    // Convert to target type
    let converted_amount = U::try_from_decimal(rounded_decimal).map_err(|_|ExchangeError::ConversionError)?;

    Ok(Monetary::new(converted_amount, self.target_currency.clone()))
}

    
    fn apply_rounding(&self, value: Decimal) -> Decimal {
        match self.context.rounding_mode() {
            RoundingMode::Up => value.ceil(),
            RoundingMode::Down => value.floor(),
            RoundingMode::HalfUp => value.round_dp_with_strategy(
                self.context.max_scale() as u32, 
                rust_decimal::RoundingStrategy::MidpointAwayFromZero
            ),
            RoundingMode::HalfDown => value.round_dp_with_strategy(
                self.context.max_scale() as u32,
                rust_decimal::RoundingStrategy::MidpointTowardZero  
            ),
            RoundingMode::HalfEven => value.round_dp_with_strategy(
                self.context.max_scale() as u32,
                rust_decimal::RoundingStrategy::MidpointNearestEven
            ),
            RoundingMode::Unnecessary => value, // No rounding,
            _ => value
        }
    }
}


/// High-performance trait for exchange rate providers
pub trait ExchangeRateProvider<T: Monetizable + Send + Sync>: Send + Sync
 {
    fn get_exchange_rate(
        &self, 
        base_currency: &Currency, 
        target_currency: &Currency
    ) -> Option<ExchangeRate<T>>;
    
    /// Batch fetch for better performance
    fn get_multiple_rates(
        &self,
        pairs: &[CurrencyPair]
    ) -> HashMap<CurrencyPair, ExchangeRate<T>> {
        HashMap::new() // Default empty implementation
    }
}




/// High-performance conversion service with fallback providers
pub struct CurrencyConversion<T: Monetizable> {
    providers: Vec<Arc<dyn ExchangeRateProvider<T>>>,
    rate_cache: RwLock<HashMap<CurrencyPair, ExchangeRate<T>>>,
    default_context: MonetaryContext,
}

impl<T: Monetizable + Send + Sync> CurrencyConversion<T> {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            rate_cache: RwLock::new(HashMap::new()),
            default_context: MonetaryContext::default(),
        }
    }
    
    pub fn with_context(context: MonetaryContext) -> Self {
        Self {
            providers: Vec::new(),
            rate_cache: RwLock::new(HashMap::new()),
            default_context: context,
        }
    }

    pub fn default_context(&self) ->  &MonetaryContext{
        &self.default_context
    }

    pub fn add_provider(&mut self, provider: Arc<dyn ExchangeRateProvider<T>>) {
        self.providers.push(provider);
    }
    
    /// Optimized conversion with direct currency code comparison
    pub fn convert(
        &self, 
        amount: &Monetary<T>, 
        target_currency: &Currency
    ) -> Result<Monetary<T>, ExchangeError> {
        // Fast path: same currency
        if amount.currency.numeric_code() == 
           target_currency.numeric_code() {
            return Ok(amount.clone());
        }

        let pair = CurrencyPair::new(&amount.currency, target_currency);
        
        // Check cache first
        {
            let cache = self.rate_cache.read().unwrap();
            if let Some(rate) = cache.get(&pair) {
                if !rate.is_expired() {
                    return rate.apply(amount);
                }
            }
        }

        // Try providers in order
        for provider in &self.providers {
            if let Some(rate) = provider.get_exchange_rate(&amount.currency, target_currency) {
                let result = rate.apply(amount);
                
                // Cache successful rate
                if result.is_ok() {
                    let mut cache = self.rate_cache.write().unwrap();
                    cache.insert(pair, rate);
                }
                
                return result;
            }
        }

        Err(ExchangeError::NoRateFound)
    }
    
    /// Cross-type conversion with rounding
    pub fn convert_to<U: Monetizable>(
        &self,
        amount: &Monetary<T>,
        target_currency: &Currency
    ) -> Result<Monetary<U>, ExchangeError> {
        // Fast path: same currency, just convert type
        if amount.currency.numeric_code() == 
           target_currency.numeric_code() {
            let converted_amount = U::try_from_decimal(
                amount.amount.try_to_decimal().map_err(|_|ExchangeError::ConversionError)?
            ).map_err(|_|ExchangeError::ConversionError)?;
            return Ok(Monetary::new(converted_amount, target_currency.clone()));
        }

        let pair = CurrencyPair::new(&amount.currency, target_currency);
        
        // Try providers in order
        for provider in &self.providers {
            if let Some(rate) = provider.get_exchange_rate(&amount.currency, target_currency) {
                return rate.apply_convert::<U>(amount);
            }
        }

        Err(ExchangeError::NoRateFound)
    }
    
    /// Batch conversion for better performance
    pub fn convert_batch(
        &self,
        amounts: &[Monetary<T>],
        target_currency: &Currency
    ) -> Vec<Result<Monetary<T>, ExchangeError>> {
        // Group by source currency for efficient batch processing
        let mut by_currency: HashMap<i32, Vec<usize>> = HashMap::new();
        for (idx, amount) in amounts.iter().enumerate() {
            by_currency
                .entry(amount.currency.numeric_code())
                .or_insert_with(Vec::new)
                .push(idx);
        }
        
        let mut results = vec![Err(ExchangeError::NoRateFound); amounts.len()];
        
        for (currency_code, indices) in by_currency {
            if currency_code == target_currency.numeric_code() {
                // Same currency - no conversion needed
                for &idx in &indices {
                    results[idx] = Ok(amounts[idx].clone());
                }
                continue;
            }
            
            // Get rate once for all amounts with this currency
            if let Some(first_idx) = indices.first() {
                match self.convert(&amounts[*first_idx], target_currency) {
                    Ok(_) => {
                        // Rate exists, convert all amounts with this currency
                        for &idx in &indices {
                            results[idx] = self.convert(&amounts[idx], target_currency);
                        }
                    }
                    Err(e) => {
                        // Rate not found, mark all as failed
                        for &idx in &indices {
                            results[idx] = Err(e.clone());
                        }
                    }
                }
            }
        }
        
        results
    }
}

impl<T: Monetizable + Send + Sync> Default for CurrencyConversion<T> {
    fn default() -> Self {
        Self::new()
    }
}












/// Extension trait to add conversion methods directly to Monetary
pub trait MoneyConversion<T: Monetizable> {
    fn convert_with_rate(&self, rate: T, target_currency: Currency) -> Monetary<T>;
    fn convert_to_type<U: Monetizable>(&self, rate: T, target_currency: Currency) -> Result<Monetary<U>, ExchangeError>;
}

impl<T: Monetizable> MoneyConversion<T> for Monetary<T> {
    fn convert_with_rate(&self, rate: T, target_currency: Currency) -> Monetary<T> {
        let new_amount = self.amount * rate;
        Monetary::new(new_amount, target_currency)
    }
    
    fn convert_to_type<U: Monetizable>(
    &self,
    rate: T,
    target_currency: Currency,
) -> Result<Monetary<U>, ExchangeError> {
    // Safely convert both amount and rate to decimal
    let amount_decimal = self.amount.try_to_decimal()
        .map_err(|_| ExchangeError::ConversionError)?;
    let rate_decimal = rate.try_to_decimal()
        .map_err(|_| ExchangeError::ConversionError)?;

    let result_decimal = amount_decimal * rate_decimal;

    let new_amount = U::try_from_decimal(result_decimal)
        .map_err(|_| ExchangeError::ConversionError)?;

    Ok(Monetary::new(new_amount, target_currency))
}
}

