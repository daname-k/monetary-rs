/// Static exchange rate provider for testing/fixed rates
use crate::core::{Monetizable, MonetaryContext};
use crate::core::currency::Currency;
use crate::exchange::base_exchange::{ExchangeRateProvider, CurrencyPair, ExchangeRate};
use std::collections::HashMap;





pub struct StaticRateProvider<T: Monetizable> {
    rates: HashMap<CurrencyPair, T>,
    context: MonetaryContext,
}

impl<T: Monetizable> StaticRateProvider<T> {
    pub fn new() -> Self {
        Self {
            rates: HashMap::new(),
            context: MonetaryContext::default(),
        }
    }
    
    pub fn with_context(context: MonetaryContext) -> Self {
        Self {
            rates: HashMap::new(),
            context,
        }
    }
    
    pub fn add_rate(&mut self, base: &Currency, target: &Currency, rate: T) {
        let pair = CurrencyPair::new(base, target);
        self.rates.insert(pair, rate);
    }
}

impl<T: Monetizable + std::marker::Sync + std::marker::Send> ExchangeRateProvider<T> for StaticRateProvider<T> {
    fn get_exchange_rate(
        &self, 
        base_currency: &Currency, 
        target_currency: &Currency
    ) -> Option<ExchangeRate<T>> {
        let pair = CurrencyPair::new(base_currency, target_currency);
        
        self.rates.get(&pair).map(|&rate| {
            ExchangeRate::new(
                base_currency.clone(),
                target_currency.clone(),
                rate
            ).with_context(self.context.clone())
        })
    }
}
