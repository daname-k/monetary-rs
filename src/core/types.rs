// bigdecimal.rs
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use crate::constants::RoundingMode;
use std::str::FromStr;

// Simplified BigDecimal implementation for demonstration
// In a real implementation, use a proper big decimal crate
#[derive(Debug, Clone, Copy, Default)]
pub struct BigDecimal {
    unscaled_value: i128, // Changed to i128 for better precision
    scale: i32,
}

impl BigDecimal {
    pub fn new(unscaled_value: i128, scale: i32) -> Self {
        Self {
            unscaled_value,
            scale,
        }
    }

    pub fn from_i64(value: i64) -> Self {
        Self {
            unscaled_value: value as i128,
            scale: 0,
        }
    }

    pub fn from_f64(value: f64, scale: i32) -> Self {
        let factor = 10_i128.pow(scale as u32);
        let unscaled_value = (value * factor as f64).round() as i128;
        Self {
            unscaled_value,
            scale,
        }
    }

    pub fn zero() -> Self {
        Self {
            unscaled_value: 0,
            scale: 0,
        }
    }

    pub fn one() -> Self {
        Self {
            unscaled_value: 1,
            scale: 0,
        }
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn unscaled_value(&self) -> i128 {
        self.unscaled_value
    }

    pub fn to_f64(&self) -> f64 {
        let divisor = 10_f64.powi(self.scale);
        self.unscaled_value as f64 / divisor
    }

    pub fn with_scale(&self, scale: i32, rounding_mode: &RoundingMode) -> Self {
        if scale == self.scale {
            return self.clone();
        }

        if scale > self.scale {
            // Increase precision (no rounding needed)
            let factor = 10_i128.pow((scale - self.scale) as u32);
            Self {
                unscaled_value: self.unscaled_value * factor,
                scale,
            }
        } else {
            // Decrease precision (rounding needed)
            let factor = 10_i128.pow((self.scale - scale) as u32);
            let unscaled_value = match rounding_mode {
                RoundingMode::HalfEven => {
                    // Banker's rounding (round to even)
                    let remainder = self.unscaled_value % factor;
                    let half = factor / 2;
                    
                    let quotient = self.unscaled_value / factor;
                    if remainder.abs() > half || (remainder.abs() == half && quotient % 2 != 0) {
                        quotient + if remainder >= 0 { 1 } else { -1 }
                    } else {
                        quotient
                    }
                }
                RoundingMode::HalfUp => {
                    let remainder = self.unscaled_value % factor;
                    let half = factor / 2;
                    let quotient = self.unscaled_value / factor;
                    
                    if remainder.abs() >= half {
                        quotient + if remainder >= 0 { 1 } else { -1 }
                    } else {
                        quotient
                    }
                }
                RoundingMode::HalfDown => {
                    let remainder = self.unscaled_value % factor;
                    let half = factor / 2;
                    let quotient = self.unscaled_value / factor;
                    
                    if remainder.abs() > half {
                        quotient + if remainder >= 0 { 1 } else { -1 }
                    } else {
                        quotient
                    }
                }
                RoundingMode::Ceiling => {
                    let quotient = self.unscaled_value / factor;
                    let remainder = self.unscaled_value % factor;
                    if remainder > 0 {
                        quotient + 1
                    } else {
                        quotient
                    }
                }
                RoundingMode::Floor => {
                    let quotient = self.unscaled_value / factor;
                    let remainder = self.unscaled_value % factor;
                    if remainder < 0 {
                        quotient - 1
                    } else {
                        quotient
                    }
                }
                RoundingMode::Down => self.unscaled_value / factor,
                RoundingMode::Up => {
                    let quotient = self.unscaled_value / factor;
                    let remainder = self.unscaled_value % factor;
                    if remainder != 0 {
                        quotient + if remainder > 0 { 1 } else { -1 }
                    } else {
                        quotient
                    }
                }
                RoundingMode::Unnecessary => {
                    if self.unscaled_value % factor != 0 {
                        panic!("Rounding necessary but RoundingMode::Unnecessary specified");
                    }
                    self.unscaled_value / factor
                }
            };

            Self {
                unscaled_value,
                scale,
            }
        }
    }

    pub fn add(&self, other: &Self, rounding_mode: &RoundingMode) -> Self {
        // Ensure both numbers have the same scale for addition
        let max_scale = self.scale.max(other.scale);
        let self_adjusted = self.with_scale(max_scale, rounding_mode);
        let other_adjusted = other.with_scale(max_scale, rounding_mode);
        
        Self {
            unscaled_value: self_adjusted.unscaled_value + other_adjusted.unscaled_value,
            scale: max_scale,
        }
    }

    pub fn subtract(&self, other: &Self, rounding_mode: &RoundingMode) -> Self {
        // Ensure both numbers have the same scale for subtraction
        let max_scale = self.scale.max(other.scale);
        let self_adjusted = self.with_scale(max_scale, rounding_mode);
        let other_adjusted = other.with_scale(max_scale, rounding_mode);
        
        Self {
            unscaled_value: self_adjusted.unscaled_value - other_adjusted.unscaled_value,
            scale: max_scale,
        }
    }

    pub fn multiply(&self, other: &Self, rounding_mode: &RoundingMode, target_scale: i32) -> Self {
        // When multiplying, scales add up
        let result_scale = self.scale + other.scale;
        let result = Self {
            unscaled_value: self.unscaled_value * other.unscaled_value,
            scale: result_scale,
        };
        
        // Adjust to target scale if needed
        result.with_scale(target_scale, rounding_mode)
    }

    pub fn divide(&self, other: &Self, rounding_mode: &RoundingMode, target_scale: i32) -> Result<Self, &'static str> {
        if other.unscaled_value == 0 {
            return Err("Division by zero");
        }

        // For better precision, scale up the dividend
        let scale_factor = 10_i128.pow((target_scale + 10) as u32);
        let scaled_dividend = self.unscaled_value * scale_factor;
        let quotient = scaled_dividend / other.unscaled_value;
        
        let result = Self {
            unscaled_value: quotient,
            scale: self.scale - other.scale + target_scale + 10,
        };
        
        Ok(result.with_scale(target_scale, rounding_mode))
    }

    pub fn negate(&self) -> Self {
        Self {
            unscaled_value: -self.unscaled_value,
            scale: self.scale,
        }
    }

    pub fn abs(&self) -> Self {
        Self {
            unscaled_value: self.unscaled_value.abs(),
            scale: self.scale,
        }
    }

    pub fn signum(&self) -> i32 {
        if self.unscaled_value > 0 {
            1
        } else if self.unscaled_value < 0 {
            -1
        } else {
            0
        }
    }

    pub fn is_zero(&self) -> bool {
        self.unscaled_value == 0
    }

    pub fn is_positive(&self) -> bool {
        self.unscaled_value > 0
    }

    pub fn is_negative(&self) -> bool {
        self.unscaled_value < 0
    }
}

impl PartialEq for BigDecimal {
    fn eq(&self, other: &Self) -> bool {
        // Convert to common scale for comparison
        let max_scale = self.scale.max(other.scale);
        let self_adjusted = self.with_scale(max_scale, &RoundingMode::HalfEven);
        let other_adjusted = other.with_scale(max_scale, &RoundingMode::HalfEven);
        
        self_adjusted.unscaled_value == other_adjusted.unscaled_value
    }
}

impl PartialOrd for BigDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Convert to common scale for comparison
        let max_scale = self.scale.max(other.scale);
        let self_adjusted = self.with_scale(max_scale, &RoundingMode::HalfEven);
        let other_adjusted = other.with_scale(max_scale, &RoundingMode::HalfEven);
        
        self_adjusted.unscaled_value.partial_cmp(&other_adjusted.unscaled_value)
    }
}

impl fmt::Display for BigDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scale == 0 {
            write!(f, "{}", self.unscaled_value)
        } else if self.scale > 0 {
            let divisor = 10_i128.pow(self.scale as u32);
            let integer_part = self.unscaled_value / divisor;
            let fraction_part = self.unscaled_value.abs() % divisor;
            
            // Handle the case where integer part is 0 but the number is negative
            if integer_part == 0 && self.unscaled_value < 0 {
                write!(
                    f,
                    "-0.{:0width$}",
                    fraction_part,
                    width = self.scale as usize
                )
            } else {
                // Normal case
                write!(
                    f,
                    "{}.{:0width$}",
                    integer_part,
                    fraction_part,
                    width = self.scale as usize
                )
            }
        } else {
            // Negative scale means multiply by 10^abs(scale)
            let multiplier = 10_i128.pow((-self.scale) as u32);
            write!(f, "{}", self.unscaled_value * multiplier)
        }
    }
}



impl FromStr for BigDecimal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty string");
        }

        let parts: Vec<&str> = s.split('.').collect();
        
        match parts.len() {
            1 => {
                // No decimal point
                let unscaled_value = parts[0].parse::<i128>().map_err(|_| "Invalid number format")?;
                Ok(Self {
                    unscaled_value,
                    scale: 0,
                })
            }
            2 => {
                // Has decimal point
                let integer_part = parts[0];
                let fraction_part = parts[1];
                
                if fraction_part.is_empty() {
                    return Err("Empty fraction part");
                }
                
                let scale = fraction_part.len() as i32;
                let fraction_value = fraction_part.parse::<i128>().map_err(|_| "Invalid fraction part")?;
                
                // Check if the whole number is negative
                let is_negative = s.starts_with('-');
                
                let unscaled_value = if integer_part == "0" || integer_part == "-0" {
                    // Handle cases like "0.123" and "-0.123"
                    if is_negative {
                        -fraction_value
                    } else {
                        fraction_value
                    }
                } else {
                    // Normal case where integer part is non-zero
                    let integer_value = integer_part.parse::<i128>().map_err(|_| "Invalid integer part")?;
                    let factor = 10_i128.pow(scale as u32);
                    
                    if integer_value < 0 {
                        integer_value * factor - fraction_value
                    } else {
                        integer_value * factor + fraction_value
                    }
                };
                
                Ok(Self {
                    unscaled_value,
                    scale,
                })
            }
            _ => Err("Invalid number format"),
        }
    }
}
















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let bd1 = BigDecimal::new(12345, 2);
        assert_eq!(bd1.unscaled_value(), 12345);
        assert_eq!(bd1.scale(), 2);
        assert_eq!(bd1.to_string(), "123.45");

        let bd2 = BigDecimal::from_i64(42);
        assert_eq!(bd2.to_string(), "42");

        let bd3 = BigDecimal::from_f64(3.14159, 5);
        assert_eq!(bd3.scale(), 5);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(BigDecimal::from_str("123").unwrap().to_string(), "123");
        assert_eq!(BigDecimal::from_str("123.45").unwrap().to_string(), "123.45");
        assert_eq!(BigDecimal::from_str("-123.45").unwrap().to_string(), "-123.45");
        assert_eq!(BigDecimal::from_str("0.001").unwrap().to_string(), "0.001");
        assert_eq!(BigDecimal::from_str("-0.001").unwrap().to_string(), "-0.001");
        assert_eq!(BigDecimal::from_str("0.0").unwrap().to_string(), "0.0");
        assert_eq!(BigDecimal::from_str("-0.0").unwrap().to_string(), "0.0"); // -0.0 should be 0.0
        assert_eq!(BigDecimal::from_str("0").unwrap().to_string(), "0");
        assert_eq!(BigDecimal::from_str("-0").unwrap().to_string(), "0");
        
        // Test edge cases with negative decimals
        let neg_small = BigDecimal::from_str("-0.001").unwrap();
        assert!(neg_small.is_negative());
        assert_eq!(neg_small.unscaled_value(), -1);
        assert_eq!(neg_small.scale(), 3);
        
        let pos_small = BigDecimal::from_str("0.001").unwrap();
        assert!(pos_small.is_positive());
        assert_eq!(pos_small.unscaled_value(), 1);
        assert_eq!(pos_small.scale(), 3);
        
        assert!(BigDecimal::from_str("").is_err());
        assert!(BigDecimal::from_str("abc").is_err());
        assert!(BigDecimal::from_str("1.2.3").is_err());
    }

    #[test]
    fn test_negative_zero_parsing() {
        // More specific test for the negative zero issue
        let test_cases = vec![
            ("0.001", 1, 3, false),
            ("-0.001", -1, 3, true),
            ("0.123", 123, 3, false),
            ("-0.123", -123, 3, true),
            ("12.34", 1234, 2, false),
            ("-12.34", -1234, 2, true),
        ];
        
        for (input, expected_unscaled, expected_scale, should_be_negative) in test_cases {
            let bd = BigDecimal::from_str(input).unwrap();
            assert_eq!(bd.unscaled_value(), expected_unscaled, "Failed for input: {}", input);
            assert_eq!(bd.scale(), expected_scale, "Failed scale for input: {}", input);
            assert_eq!(bd.is_negative(), should_be_negative, "Failed sign for input: {}", input);
            assert_eq!(bd.to_string(), input, "Failed string representation for input: {}", input);
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(BigDecimal::new(12345, 2).to_string(), "123.45");
        assert_eq!(BigDecimal::new(-12345, 2).to_string(), "-123.45");
        assert_eq!(BigDecimal::new(123, 0).to_string(), "123");
        assert_eq!(BigDecimal::new(123, -1).to_string(), "1230");
        assert_eq!(BigDecimal::new(5, 3).to_string(), "0.005");
    }

    #[test]
    fn test_with_scale() {
        let bd = BigDecimal::new(12345, 2); // 123.45
        
        // Increase scale
        let bd_scaled = bd.with_scale(4, &RoundingMode::HalfEven);
        assert_eq!(bd_scaled.to_string(), "123.4500");
        
        // Decrease scale with rounding
        let bd_rounded = bd.with_scale(1, &RoundingMode::HalfEven);
        assert_eq!(bd_rounded.to_string(), "123.4"); // 123.45 -> 123.4
        
        // Test different rounding modes
        let bd_test = BigDecimal::new(12356, 2); // 123.56
        assert_eq!(bd_test.with_scale(1, &RoundingMode::HalfEven).to_string(), "123.6");
        assert_eq!(bd_test.with_scale(1, &RoundingMode::Floor).to_string(), "123.5");
        assert_eq!(bd_test.with_scale(1, &RoundingMode::Ceiling).to_string(), "123.6");
    }

    #[test]
    fn test_arithmetic_operations() {
        let bd1 = BigDecimal::from_str("123.45").unwrap();
        let bd2 = BigDecimal::from_str("67.89").unwrap();
        
        // Addition
        let sum = bd1.add(&bd2, &RoundingMode::HalfEven);
        assert_eq!(sum.to_string(), "191.34");
        
        // Subtraction
        let diff = bd1.subtract(&bd2, &RoundingMode::HalfEven);
        assert_eq!(diff.to_string(), "55.56");
        
        // Multiplication: 123.45 * 67.89 = 8381.0205, rounded to 2 decimals = 8381.02
        let product = bd1.multiply(&bd2, &RoundingMode::HalfEven, 2);
        assert_eq!(product.to_string(), "8381.02");
        
        // Division: 123.45 / 67.89 ≈ 1.8183826778612462, rounded to 2 decimals = 1.82
        let quotient = bd1.divide(&bd2, &RoundingMode::HalfEven, 2).unwrap();
        assert_eq!(quotient.to_string(), "1.82");
    }

    #[test]
    fn test_comparison() {
        let bd1 = BigDecimal::from_str("123.45").unwrap();
        let bd2 = BigDecimal::from_str("123.450").unwrap(); // Same value, different scale
        let bd3 = BigDecimal::from_str("123.46").unwrap();
        
        assert_eq!(bd1, bd2);
        assert!(bd1 < bd3);
        assert!(bd3 > bd1);
    }

    #[test]
    fn test_sign_operations() {
        let bd_pos = BigDecimal::from_str("123.45").unwrap();
        let bd_neg = BigDecimal::from_str("-123.45").unwrap();
        let bd_zero = BigDecimal::zero();
        
        assert_eq!(bd_pos.signum(), 1);
        assert_eq!(bd_neg.signum(), -1);
        assert_eq!(bd_zero.signum(), 0);
        
        assert!(bd_pos.is_positive());
        assert!(bd_neg.is_negative());
        assert!(bd_zero.is_zero());
        
        assert_eq!(bd_pos.negate(), bd_neg);
        assert_eq!(bd_neg.abs(), bd_pos);
    }

    #[test]
    fn test_rounding_modes() {
        let bd = BigDecimal::new(12355, 3); // 12.355
        
        // Test HalfEven (banker's rounding)
        assert_eq!(bd.with_scale(2, &RoundingMode::HalfEven).to_string(), "12.36");
        
        // Test HalfUp
        assert_eq!(bd.with_scale(2, &RoundingMode::HalfUp).to_string(), "12.36");
        
        // Test HalfDown
        assert_eq!(bd.with_scale(2, &RoundingMode::HalfDown).to_string(), "12.35");
        
        // Test Up
        assert_eq!(bd.with_scale(2, &RoundingMode::Up).to_string(), "12.36");
        
        // Test Down
        assert_eq!(bd.with_scale(2, &RoundingMode::Down).to_string(), "12.35");
        
        // Test Ceiling
        assert_eq!(bd.with_scale(2, &RoundingMode::Ceiling).to_string(), "12.36");
        
        // Test Floor
        assert_eq!(bd.with_scale(2, &RoundingMode::Floor).to_string(), "12.35");
    }

    #[test]
    fn test_negative_rounding() {
        let bd = BigDecimal::new(-12355, 3); // -12.355
        
        assert_eq!(bd.with_scale(2, &RoundingMode::HalfEven).to_string(), "-12.36");
        assert_eq!(bd.with_scale(2, &RoundingMode::Ceiling).to_string(), "-12.35");
        assert_eq!(bd.with_scale(2, &RoundingMode::Floor).to_string(), "-12.36");
        assert_eq!(bd.with_scale(2, &RoundingMode::Up).to_string(), "-12.36");
        assert_eq!(bd.with_scale(2, &RoundingMode::Down).to_string(), "-12.35");
    }

    #[test]
    fn test_division_by_zero() {
        let bd1 = BigDecimal::from_str("123.45").unwrap();
        let bd_zero = BigDecimal::zero();
        
        assert!(bd1.divide(&bd_zero, &RoundingMode::HalfEven, 2).is_err());
    }

    #[test]
    #[should_panic(expected = "Rounding necessary but RoundingMode::Unnecessary specified")]
    fn test_unnecessary_rounding_panic() {
        let bd = BigDecimal::new(12355, 3); // 12.355
        bd.with_scale(2, &RoundingMode::Unnecessary);
    }

    #[test]
    fn test_unnecessary_rounding_no_panic() {
        let bd = BigDecimal::new(12350, 3); // 12.350
        let result = bd.with_scale(2, &RoundingMode::Unnecessary);
        assert_eq!(result.to_string(), "12.35");
    }

    #[test]
    fn test_edge_cases() {
        // Test zero
        let zero = BigDecimal::zero();
        assert_eq!(zero.to_string(), "0");
        assert!(zero.is_zero());
        
        // Test one
        let one = BigDecimal::one();
        assert_eq!(one.to_string(), "1");
        
        // Test very small numbers
        let small = BigDecimal::new(1, 10);
        assert_eq!(small.to_string(), "0.0000000001");
        
        // Test arithmetic with different scales
        let bd1 = BigDecimal::new(123, 1); // 12.3
        let bd2 = BigDecimal::new(4567, 3); // 4.567
        let sum = bd1.add(&bd2, &RoundingMode::HalfEven);
        assert_eq!(sum.to_string(), "16.867");
    }

    #[test]
    fn test_precision_arithmetic() {
        // Test high precision operations
        let bd1 = BigDecimal::from_str("1.23456789").unwrap();
        let bd2 = BigDecimal::from_str("9.87654321").unwrap();
        
        let sum = bd1.add(&bd2, &RoundingMode::HalfEven);
        assert_eq!(sum.to_string(), "11.11111110");
        
        let product = bd1.multiply(&bd2, &RoundingMode::HalfEven, 8);
        assert_eq!(product.to_string(), "12.19326311");
    }


    #[test]
    fn test_scale_preservation() {
        // Test that operations preserve appropriate scale
        let bd1 = BigDecimal::new(12345, 2); // 123.45
        let bd2 = BigDecimal::new(6789, 3);  // 6.789
        
        let sum = bd1.add(&bd2, &RoundingMode::HalfEven);
        assert_eq!(sum.scale(), 3); // Should use the higher scale
        assert_eq!(sum.to_string(), "130.239");
        
        let product = bd1.multiply(&bd2, &RoundingMode::HalfEven, 4);
        assert_eq!(product.scale(), 4);
        assert_eq!(product.to_string(), "838.4121"); // 123.45 * 6.789 = 838.41205, rounded to 4 decimals
    }

}