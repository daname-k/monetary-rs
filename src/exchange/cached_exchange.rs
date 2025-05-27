/// Simple implementation of exchange rate provider
use crate::core::{Monetary, Monetizable, MonetaryContext};
use crate::core::currency::Currency;
use crate::core::currency_unit::CurrencyUnit;
use crate::constants::RoundingMode;
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use rust_decimal::Decimal;
use crate::exchange::base_exchange::{ExchangeRateProvider, CurrencyPair, ExchangeRate};

/// Fast in-memory cache with automatic cleanup
pub struct CachedExchangeRateProvider<T: Monetizable + Send + Sync> {
    cache: RwLock<HashMap<CurrencyPair, ExchangeRate<T>>>,
    upstream_provider: Arc<dyn ExchangeRateProvider<T>>,
    default_ttl: Duration,
}

impl<T: Monetizable + Send + Sync> CachedExchangeRateProvider<T> {
    pub fn new(
        upstream_provider: Arc<dyn ExchangeRateProvider<T>>,
        default_ttl: Duration
    ) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            upstream_provider,
            default_ttl,
        }
    }
    
    fn cleanup_expired(&self) {
        let mut cache = self.cache.write().unwrap();
        cache.retain(|_, rate| !rate.is_expired());
    }
}


impl<T: Monetizable + Send + Sync> ExchangeRateProvider<T> for CachedExchangeRateProvider<T> {
    fn get_exchange_rate(
        &self, 
        base_currency: &Currency, 
        target_currency: &Currency
    ) -> Option<ExchangeRate<T>> {
        let pair = CurrencyPair::new(base_currency, target_currency);
        
        // Fast read path
        {
            let cache = self.cache.read().unwrap();
            if let Some(rate) = cache.get(&pair) {
                if !rate.is_expired() {
                    return Some(rate.clone());
                }
            }
        }
        
        // Slow path: fetch from upstream and cache
        if let Some(rate) = self.upstream_provider.get_exchange_rate(base_currency, target_currency) {
            let  _rate = rate.clone().with_ttl(self.default_ttl);
            
            let mut cache = self.cache.write().unwrap();
            cache.insert(pair, _rate);
            
            // Periodic cleanup (every 100th access)
            if cache.len() % 100 == 0 {
                cache.retain(|_, r| !r.is_expired());
            }
            
            Some(rate)
        } else {
            None
        }
    }
}

