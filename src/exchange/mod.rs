pub mod base_exchange;
pub mod cached_exchange;
pub mod static_exchange;




#[cfg(test)]
mod tests {
    use super::*;
    use base_exchange::{CurrencyConversion,CurrencyPair};
    use crate::exchange::base_exchange::{ExchangeRateProvider, ExchangeRate, MoneyConversion};
    use crate::errors::ExchangeError;
    use crate::core::{Monetary, Monetizable, MonetaryContext};
    use crate::core::currency::Currency;
    use crate::core::currency_unit::CurrencyUnit;
    use rust_decimal::Decimal;
    use crate::exchange::static_exchange::StaticRateProvider;
    use crate::exchange::cached_exchange::CachedExchangeRateProvider;
    use crate::constants::RoundingMode;
    use std::time::{Duration};
    use std::thread;
    use std::sync::Arc;

    // Helper function to create test currencies
    fn create_test_currency(code: &str, numeric: i32) -> Currency {
        let unit = CurrencyUnit::new(code, numeric, 2, code);
        Currency::new(unit, "$")
    }

    // Helper function to create test money
    fn create_test_money(amount: f64, currency: Currency) -> Monetary<Decimal> {
        Monetary::new(Decimal::try_from_f64(amount).unwrap(), currency)
    }

    // Mock provider for testing caching behavior
    struct MockProvider<T: Monetizable> {
        call_count: Arc<std::sync::Mutex<usize>>,
        rates: std::collections::HashMap<CurrencyPair, T>,
    }

    impl<T: Monetizable> MockProvider<T> {
        fn new() -> Self {
            Self {
                call_count: Arc::new(std::sync::Mutex::new(0)),
                rates: std::collections::HashMap::new(),
            }
        }

        fn with_rate(mut self, base: &Currency, target: &Currency, rate: T) -> Self {
            let pair = CurrencyPair::new(base, target);
            self.rates.insert(pair, rate);
            self
        }

        fn get_call_count(&self) -> usize {
            *self.call_count.lock().unwrap()
        }
    }

    impl<T: Monetizable + Send + Sync> ExchangeRateProvider<T> for MockProvider<T> {
        fn get_exchange_rate(
            &self,
            base_currency: &Currency,
            target_currency: &Currency,
        ) -> Option<ExchangeRate<T>> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;

            let pair = CurrencyPair::new(base_currency, target_currency);
            self.rates.get(&pair).map(|&rate| {
                ExchangeRate::new(
                    base_currency.clone(),
                    target_currency.clone(),
                    rate,
                )
            })
        }
    }

    

    
    #[test]
    fn test_same_currency_conversion() {
        let usd = create_test_currency("USD", 840);
        let amount = Monetary::new(Decimal::from(100), usd.clone());
        
        let converter = CurrencyConversion::<Decimal>::new();
        let result = converter.convert(&amount, &usd).unwrap();
        
        assert_eq!(result.amount, Decimal::from(100));
    }
    
    #[test]
    fn test_static_rate_provider() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        
        let mut provider = static_exchange::StaticRateProvider::new();
        provider.add_rate(&usd, &eur, Decimal::try_from_f64(0.85).unwrap());
        
        let rate = provider.get_exchange_rate(&usd, &eur).unwrap();
        assert_eq!(*rate.get_factor(), Decimal::try_from_f64(0.85).unwrap());
    }
    
    #[test]
    fn test_currency_pair_equality() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        
        let pair1 = CurrencyPair::new(&usd, &eur);
        let pair2 = CurrencyPair::new(&usd, &eur);
        
        assert_eq!(pair1, pair2);
    }
    
    #[test] 
    fn test_cross_type_conversion() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        
        let amount_decimal = Monetary::new(Decimal::from(100), usd.clone());
        let rate = ExchangeRate::new(usd, eur.clone(), Decimal::try_from_f64(0.85).unwrap());
        
        let result: Monetary<f64> = rate.apply_convert(&amount_decimal).unwrap();
        assert!((result.amount - 85.0).abs() < 0.00001);
    }



    #[test]
    fn test_currency_pair_creation() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        
        let pair1 = CurrencyPair::new(&usd, &eur);
        let pair2 = CurrencyPair::new(&usd, &eur);
        let pair3 = CurrencyPair::new(&eur, &usd);
        
        assert_eq!(pair1, pair2);
        assert_ne!(pair1, pair3);
    }

    #[test]
    fn test_exchange_rate_basic_functionality() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let rate = ExchangeRate::new(usd.clone(), eur.clone(), rate_value);
        
        assert_eq!(rate.get_base_currency(), &usd);
        assert_eq!(rate.get_target_currency(), &eur);
        assert_eq!(*rate.get_factor(), rate_value);
        assert!(!rate.is_expired());
        assert!(rate.get_ttl().is_none());
    }

    #[test]
    fn test_exchange_rate_with_ttl() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        let ttl = Duration::from_millis(50);
        
        let rate = ExchangeRate::new(usd, eur, rate_value).with_ttl(ttl);
        
        assert!(!rate.is_expired());
        assert_eq!(rate.get_ttl(), &Some(ttl));
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(60));
        assert!(rate.is_expired());
    }

    #[test]
    fn test_exchange_rate_apply_same_type() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let rate = ExchangeRate::new(usd.clone(), eur.clone(), rate_value);
        let money = create_test_money(100.0, usd);
        
        let result = rate.apply(&money).unwrap();
        assert_eq!(result.amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(result.currency, eur);
    }

    #[test]
    fn test_exchange_rate_currency_mismatch() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let gbp = create_test_currency("GBP", 826);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let rate = ExchangeRate::new(usd, eur, rate_value);
        let money = create_test_money(100.0, gbp);
        
        let result = rate.apply(&money);
        assert_eq!(result.unwrap_err(), ExchangeError::CurrencyMismatch);
    }

    #[test]
    fn test_exchange_rate_expired() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        let ttl = Duration::from_millis(10);
        
        let rate = ExchangeRate::new(usd.clone(), eur, rate_value).with_ttl(ttl);
        let money = create_test_money(100.0, usd);
        
        thread::sleep(Duration::from_millis(20));
        
        let result = rate.apply(&money);
        assert_eq!(result.unwrap_err(), ExchangeError::ExpiredRate);
    }

    #[test]
    fn test_exchange_rate_apply_convert_cross_type() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let rate = ExchangeRate::new(usd.clone(), eur.clone(), rate_value);
        let money = create_test_money(100.0, usd);
        
        // Convert from Decimal to Decimal (same type for this test)
        let result: Result<Monetary<Decimal>, ExchangeError> = rate.apply_convert(&money);
        assert!(result.is_ok());
        
        let converted = result.unwrap();
        assert_eq!(converted.amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(converted.currency, eur);
    }

    #[test]
    fn test_exchange_rate_rounding_modes() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.8555).unwrap(); // Will result in 85.55
        
        // Test HalfUp rounding
        let context_half_up = MonetaryContext::builder()
            .with_rounding_mode(RoundingMode::HalfUp)
            .with_max_scale(1).build();
        
        let rate = ExchangeRate::new(usd.clone(), eur.clone(), rate_value)
            .with_context(context_half_up);
        let money = create_test_money(100.0, usd.clone());
        
        let result: Result<Monetary<Decimal>, ExchangeError> = rate.apply_convert(&money);
        assert!(result.is_ok());
        
        // Test Down rounding
        let context_down = MonetaryContext::builder()
            .with_rounding_mode(RoundingMode::Down)
            .with_max_scale(1).build();
        
        let rate_down = ExchangeRate::new(usd.clone(), eur.clone(), rate_value)
            .with_context(context_down);
        
        let result_down: Result<Monetary<Decimal>, ExchangeError> = rate_down.apply_convert(&money);
        assert!(result_down.is_ok());
    }

    #[test]
    fn test_static_rate_provider_basic() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mut provider = StaticRateProvider::new();
        provider.add_rate(&usd, &eur, rate_value);
        
        let rate = provider.get_exchange_rate(&usd, &eur).unwrap();
        assert_eq!(*rate.get_factor(), rate_value);
        assert_eq!(rate.get_base_currency(), &usd);
        assert_eq!(rate.get_target_currency(), &eur);
    }

    #[test]
    fn test_static_rate_provider_no_rate() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let gbp = create_test_currency("GBP", 826);
        
        let mut provider = StaticRateProvider::new();
        provider.add_rate(&usd, &eur, Decimal::try_from_f64(0.85).unwrap());
        
        let rate = provider.get_exchange_rate(&usd, &gbp);
        assert!(rate.is_none());
    }

    #[test]
    fn test_static_rate_provider_with_context() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let context = MonetaryContext::builder()
            .with_rounding_mode(RoundingMode::HalfUp)
            .with_max_scale(2).build();
        
        let mut provider = StaticRateProvider::with_context(context.clone());
        provider.add_rate(&usd, &eur, rate_value);
        
        let rate = provider.get_exchange_rate(&usd, &eur).unwrap();
        assert_eq!(rate.get_context().rounding_mode(), context.rounding_mode());
        assert_eq!(rate.get_context().max_scale(), context.max_scale());
    }

    #[test]
    fn test_cached_provider_basic_functionality() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mock_provider = MockProvider::new()
            .with_rate(&usd, &eur, rate_value);
        
        let cached_provider = CachedExchangeRateProvider::new(
            Arc::new(mock_provider),
            Duration::from_secs(300),
        );
        
        // First call should hit upstream
        let rate1 = cached_provider.get_exchange_rate(&usd, &eur).unwrap();
        assert_eq!(*rate1.get_factor(), rate_value);
        
        // Second call should hit cache
        let rate2 = cached_provider.get_exchange_rate(&usd, &eur).unwrap();
        assert_eq!(*rate2.get_factor(), rate_value);
    }

    #[test]
    fn test_cached_provider_cache_behavior() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mock_provider = Arc::new(MockProvider::new()
            .with_rate(&usd, &eur, rate_value));
        
        let mock_clone = Arc::clone(&mock_provider);
        let cached_provider = CachedExchangeRateProvider::new(
            mock_provider,
            Duration::from_secs(300),
        );
        
        // Multiple calls should only hit upstream once
        for _ in 0..5 {
            let _rate = cached_provider.get_exchange_rate(&usd, &eur);
        }
        
        // Only first call should have gone to upstream
        assert_eq!(mock_clone.get_call_count(), 1);
    }

    #[test]
    fn test_cached_provider_ttl_expiration() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mock_provider = Arc::new(MockProvider::new()
            .with_rate(&usd, &eur, rate_value));
        
        let mock_clone = Arc::clone(&mock_provider);
        let cached_provider = CachedExchangeRateProvider::new(
            mock_provider,
            Duration::from_millis(50),
        );
        
        // First call
        let _rate1 = cached_provider.get_exchange_rate(&usd, &eur);
        assert_eq!(mock_clone.get_call_count(), 1);
        
        // Second call within TTL
        let _rate2 = cached_provider.get_exchange_rate(&usd, &eur);
        assert_eq!(mock_clone.get_call_count(), 1);
        
        // Wait for expiration
        thread::sleep(Duration::from_millis(60));
        
        // Third call after expiration
        let _rate3 = cached_provider.get_exchange_rate(&usd, &eur);
        assert_eq!(mock_clone.get_call_count(), 2);
    }

    #[test]
    fn test_currency_conversion_service_basic() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mut provider = StaticRateProvider::new();
        provider.add_rate(&usd, &eur, rate_value);
        
        let mut conversion_service = CurrencyConversion::new();
        conversion_service.add_provider(Arc::new(provider));
        
        let money = create_test_money(100.0, usd);
        let result = conversion_service.convert(&money, &eur).unwrap();
        
        assert_eq!(result.amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(result.currency, eur);
    }

    #[test]
    fn test_currency_conversion_same_currency() {
        let usd = create_test_currency("USD", 840);
        
        let conversion_service = CurrencyConversion::<Decimal>::new();
        let money = create_test_money(100.0, usd.clone());
        let result = conversion_service.convert(&money, &usd).unwrap();
        
        // Same currency should return identical money
        assert_eq!(result.amount, money.amount);
        assert_eq!(result.currency, money.currency);
    }

    #[test]
    fn test_currency_conversion_no_rate_found() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        
        let conversion_service = CurrencyConversion::<Decimal>::new();
        let money = create_test_money(100.0, usd);
        let result = conversion_service.convert(&money, &eur);
        
        assert_eq!(result.unwrap_err(), ExchangeError::NoRateFound);
    }

    #[test]
    fn test_currency_conversion_fallback_providers() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let gbp = create_test_currency("GBP", 826);
        
        // First provider only has USD->EUR
        let mut provider1 = StaticRateProvider::new();
        provider1.add_rate(&usd, &eur, Decimal::try_from_f64(0.85).unwrap());
        
        // Second provider only has USD->GBP
        let mut provider2 = StaticRateProvider::new();
        provider2.add_rate(&usd, &gbp, Decimal::try_from_f64(0.75).unwrap());
        
        let mut conversion_service = CurrencyConversion::new();
        conversion_service.add_provider(Arc::new(provider1));
        conversion_service.add_provider(Arc::new(provider2));
        
        let money = create_test_money(100.0, usd.clone());
        
        // Should find EUR rate from first provider
        let eur_result = conversion_service.convert(&money, &eur).unwrap();
        assert_eq!(eur_result.amount, Decimal::try_from_f64(85.0).unwrap());
        
        // Should find GBP rate from second provider
        let gbp_result = conversion_service.convert(&money, &gbp).unwrap();
        assert_eq!(gbp_result.amount, Decimal::try_from_f64(75.0).unwrap());
    }

    #[test]
    fn test_currency_conversion_batch() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let gbp = create_test_currency("GBP", 826);
        
        let mut provider = StaticRateProvider::new();
        provider.add_rate(&usd, &eur, Decimal::try_from_f64(0.85).unwrap());
        provider.add_rate(&gbp, &eur, Decimal::try_from_f64(1.15).unwrap());
        
        let mut conversion_service = CurrencyConversion::new();
        conversion_service.add_provider(Arc::new(provider));
        
        let amounts = vec![
            create_test_money(100.0, usd.clone()),
            create_test_money(200.0, usd),
            create_test_money(50.0, gbp),
            create_test_money(75.0, eur.clone()), // Same currency
        ];
        
        let results = conversion_service.convert_batch(&amounts, &eur);
        
        assert_eq!(results.len(), 4);
        assert_eq!(results[0].as_ref().unwrap().amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(results[1].as_ref().unwrap().amount, Decimal::try_from_f64(170.0).unwrap());
        assert_eq!(results[2].as_ref().unwrap().amount, Decimal::try_from_f64(57.5).unwrap());
        assert_eq!(results[3].as_ref().unwrap().amount, Decimal::try_from_f64(75.0).unwrap());
    }

    #[test]
    fn test_money_conversion_extension_trait() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate = Decimal::try_from_f64(0.85).unwrap();
        
        let money = create_test_money(100.0, usd);
        let converted = money.convert_with_rate(rate, eur.clone());
        
        assert_eq!(converted.amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(converted.currency, eur);
    }

    #[test]
    fn test_money_conversion_cross_type() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate = Decimal::try_from_f64(0.85).unwrap();
        
        let money = create_test_money(100.0, usd);
        let result: Result<Monetary<Decimal>, ExchangeError> = money.convert_to_type(rate, eur.clone());
        
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted.amount, Decimal::try_from_f64(85.0).unwrap());
        assert_eq!(converted.currency, eur);
    }

    #[test]
    fn test_exchange_error_display() {
        let errors = vec![
            ExchangeError::CurrencyMismatch,
            ExchangeError::NoRateFound,
            ExchangeError::ExpiredRate,
            ExchangeError::InvalidRate,
            ExchangeError::ProviderError,
            ExchangeError::ConversionError,
        ];
        
        for error in errors {
            let error_string = format!("{}", error);
            assert!(!error_string.is_empty());
        }
    }

    #[test]
    fn test_complex_conversion_scenario() {
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let gbp = create_test_currency("GBP", 826);
        let jpy = create_test_currency("JPY", 392);
        
        // Create a more complex provider setup
        let mut static_provider = StaticRateProvider::new();
        static_provider.add_rate(&usd, &eur, Decimal::try_from_f64(0.85).unwrap());
        static_provider.add_rate(&eur, &gbp, Decimal::try_from_f64(0.87).unwrap());
        static_provider.add_rate(&usd, &jpy, Decimal::try_from_f64(150.0).unwrap());
        
        let cached_provider = CachedExchangeRateProvider::new(
            Arc::new(static_provider),
            Duration::from_secs(300),
        );
        
        let mut conversion_service = CurrencyConversion::new();
        conversion_service.add_provider(Arc::new(cached_provider));
        
        // Test multiple conversions
        let usd_money = create_test_money(1000.0, usd.clone());
        
        let eur_result = conversion_service.convert(&usd_money, &eur).unwrap();
        assert_eq!(eur_result.amount, Decimal::try_from_f64(850.0).unwrap());
        
        let jpy_result = conversion_service.convert(&usd_money, &jpy).unwrap();
        assert_eq!(jpy_result.amount, Decimal::try_from_f64(150000.0).unwrap());
        
        // Test chained conversion (would need multiple services in real scenario)
        let gbp_from_eur = conversion_service.convert(&eur_result, &gbp).unwrap();
        assert_eq!(gbp_from_eur.amount, Decimal::try_from_f64(739.5).unwrap());
    }

    #[test]
    fn test_concurrent_cache_access() {
        use std::sync::Arc;
        use std::thread;
        
        let usd = create_test_currency("USD", 840);
        let eur = create_test_currency("EUR", 978);
        let rate_value = Decimal::try_from_f64(0.85).unwrap();
        
        let mock_provider = MockProvider::new()
            .with_rate(&usd, &eur, rate_value);
        
        let cached_provider = Arc::new(CachedExchangeRateProvider::new(
            Arc::new(mock_provider),
            Duration::from_secs(300),
        ));
        
        let mut handles = vec![];
        
        // Spawn multiple threads to access the cache concurrently
        for _ in 0..10 {
            let provider = Arc::clone(&cached_provider);
            let usd_clone = usd.clone();
            let eur_clone = eur.clone();
            
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _rate = provider.get_exchange_rate(&usd_clone, &eur_clone);
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Cache should still work after concurrent access
        let final_rate = cached_provider.get_exchange_rate(&usd, &eur);
        assert!(final_rate.is_some());
    }

    #[test]
    fn test_conversion_service_with_context() {
        let _usd = create_test_currency("USD", 840);
        let _eur = create_test_currency("EUR", 978);
        
        let context = MonetaryContext::builder()
            .with_rounding_mode(RoundingMode::HalfUp)
            .with_max_scale(2)
            .build();
        
        let mut _provider:StaticRateProvider<Decimal> = StaticRateProvider::with_context(context.clone());

        let rate = Decimal::try_from_f64(0.85).unwrap();
        
        _provider.add_rate(&_usd, &_eur, rate);

        let conversion_service: CurrencyConversion<Decimal> = CurrencyConversion::with_context(context);
        

        // Test that the service uses the provided context
        assert_eq!(
            *conversion_service.default_context().rounding_mode(),
            RoundingMode::HalfUp
        );
        assert_eq!(conversion_service.default_context().max_scale(), 2);

        // assert_eq!(conversion_service.convert(100.0, &_eur).to, Decimal::try_from_f64(85.0))
    }
}