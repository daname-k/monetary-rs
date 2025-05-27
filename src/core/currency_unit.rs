// Rust Monetary API Implementation
// Inspired by JSR 354 (Java Monetary API)

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use std::sync::Arc;

// ===== Currency =====

/// Represents a currency, similar to javax.money.CurrencyUnit
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyUnit {
    code: String,
    numeric_code: i32,
    default_fraction_digits: i32,
    display_name: String,
}

impl CurrencyUnit {
    pub fn new(
        code: &str,
        numeric_code: i32,
        default_fraction_digits: i32,
        display_name: &str,
    ) -> Self {
        Self {
            code: code.to_string(),
            numeric_code,
            default_fraction_digits,
            display_name: display_name.to_string(),
        }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_numeric_code(&self) -> i32 {
        self.numeric_code
    }

    pub fn get_default_fraction_digits(&self) -> i32 {
        self.default_fraction_digits
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
}

impl fmt::Display for CurrencyUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}
