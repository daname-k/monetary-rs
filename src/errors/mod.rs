use std::{error, fmt};

/// Currency-specific errors with enhanced functionality
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrencyError {
    /// Unknown or unsupported currency code
    UnknownCurrency { 
        code: String,
        context: Option<String>,
    },
    /// Invalid currency format or parsing error
    InvalidFormat { 
        message: String,
        input: Option<String>,
    },
    /// Currency mismatch in operations
    CurrencyMismatch { 
        expected: String,
        actual: String,
        operation: Option<String>,
    },
    /// Conversion error between currencies
    ConversionError {
        from: String,
        to: String,
        reason: String,
    },
    /// Invalid currency amount
    InvalidAmount {
        amount: String,
        reason: String,
    },
}

impl CurrencyError {
    /// Create a new UnknownCurrency error
    pub fn unknown_currency(code: impl Into<String>) -> Self {
        Self::UnknownCurrency {
            code: code.into(),
            context: None,
        }
    }

    /// Create a new UnknownCurrency error with context
    pub fn unknown_currency_with_context(code: impl Into<String>, context: impl Into<String>) -> Self {
        Self::UnknownCurrency {
            code: code.into(),
            context: Some(context.into()),
        }
    }

    /// Create a new InvalidFormat error
    pub fn invalid_format(message: impl Into<String>) -> Self {
        Self::InvalidFormat {
            message: message.into(),
            input: None,
        }
    }

    /// Create a new InvalidFormat error with input
    pub fn invalid_format_with_input(message: impl Into<String>, input: impl Into<String>) -> Self {
        Self::InvalidFormat {
            message: message.into(),
            input: Some(input.into()),
        }
    }

    /// Create a new CurrencyMismatch error
    pub fn currency_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::CurrencyMismatch {
            expected: expected.into(),
            actual: actual.into(),
            operation: None,
        }
    }

    /// Create a new CurrencyMismatch error with operation context
    pub fn currency_mismatch_with_operation(
        expected: impl Into<String>, 
        actual: impl Into<String>,
        operation: impl Into<String>
    ) -> Self {
        Self::CurrencyMismatch {
            expected: expected.into(),
            actual: actual.into(),
            operation: Some(operation.into()),
        }
    }

    /// Create a new ConversionError
    pub fn conversion_error(
        from: impl Into<String>, 
        to: impl Into<String>, 
        reason: impl Into<String>
    ) -> Self {
        Self::ConversionError {
            from: from.into(),
            to: to.into(),
            reason: reason.into(),
        }
    }

    /// Create a new InvalidAmount error
    pub fn invalid_amount(amount: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidAmount {
            amount: amount.into(),
            reason: reason.into(),
        }
    }

    /// Get the error category as a string
    pub fn category(&self) -> &'static str {
        match self {
            Self::UnknownCurrency { .. } => "UnknownCurrency",
            Self::InvalidFormat { .. } => "InvalidFormat",
            Self::CurrencyMismatch { .. } => "CurrencyMismatch",
            Self::ConversionError { .. } => "ConversionError",
            Self::InvalidAmount { .. } => "InvalidAmount",
        }
    }

    /// Check if this is a recoverable error
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::UnknownCurrency { .. } => false,
            Self::InvalidFormat { .. } => false,
            Self::CurrencyMismatch { .. } => false,
            Self::ConversionError { .. } => true, // Might retry with different rates
            Self::InvalidAmount { .. } => false,
        }
    }
}

impl fmt::Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCurrency { code, context } => {
                if let Some(ctx) = context {
                    write!(f, "Unknown currency code '{}' in context: {}", code, ctx)
                } else {
                    write!(f, "Unknown currency code: {}", code)
                }
            }
            Self::InvalidFormat { message, input } => {
                if let Some(inp) = input {
                    write!(f, "Invalid currency format: {} (input: '{}')", message, inp)
                } else {
                    write!(f, "Invalid currency format: {}", message)
                }
            }
            Self::CurrencyMismatch { expected, actual, operation } => {
                if let Some(op) = operation {
                    write!(f, "Currency mismatch in {}: expected '{}', got '{}'", op, expected, actual)
                } else {
                    write!(f, "Currency mismatch: expected '{}', got '{}'", expected, actual)
                }
            }
            Self::ConversionError { from, to, reason } => {
                write!(f, "Currency conversion error from '{}' to '{}': {}", from, to, reason)
            }
            Self::InvalidAmount { amount, reason } => {
                write!(f, "Invalid currency amount '{}': {}", amount, reason)
            }
        }
    }
}

impl error::Error for CurrencyError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Return the source of the error if any
        None
    }
}

// Convenience type alias for Results
pub type CurrencyResult<T> = Result<T, CurrencyError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_currency() {
        let err = CurrencyError::unknown_currency("XYZ");
        assert_eq!(err.category(), "UnknownCurrency");
        assert!(!err.is_recoverable());
        assert_eq!(err.to_string(), "Unknown currency code: XYZ");
    }

    #[test]
    fn test_currency_mismatch_with_operation() {
        let err = CurrencyError::currency_mismatch_with_operation("USD", "EUR", "addition");
        assert_eq!(err.category(), "CurrencyMismatch");
        assert_eq!(err.to_string(), "Currency mismatch in addition: expected 'USD', got 'EUR'");
    }

    #[test]
    fn test_conversion_error() {
        let err = CurrencyError::conversion_error("USD", "EUR", "No exchange rate available");
        assert!(err.is_recoverable());
        assert_eq!(err.to_string(), "Currency conversion error from 'USD' to 'EUR': No exchange rate available");
    }

    #[test]
    fn test_invalid_format_with_input() {
        let err = CurrencyError::invalid_format_with_input("Expected numeric value", "$abc");
        assert_eq!(err.to_string(), "Invalid currency format: Expected numeric value (input: '$abc')");
    }
}






/// Specialized error type for better performance
#[derive(Debug, PartialEq, Clone)]
pub enum ExchangeError {
    CurrencyMismatch,
    NoRateFound,
    ExpiredRate,
    InvalidRate,
    ProviderError,
    ConversionError,
}

impl fmt::Display for ExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExchangeError::CurrencyMismatch => write!(f, "Currency mismatch for exchange rate"),
            ExchangeError::NoRateFound => write!(f, "No exchange rate found"),
            ExchangeError::ExpiredRate => write!(f, "Exchange rate has expired"),
            ExchangeError::InvalidRate => write!(f, "Invalid exchange rate"),
            ExchangeError::ProviderError => write!(f, "Exchange rate provider error"),
            ExchangeError::ConversionError => write!(f, "Type conversion error"),
        }
    }
}

impl error::Error for ExchangeError {}



