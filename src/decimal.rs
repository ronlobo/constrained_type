//! Error types for the crate

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};
use crate::error::{ConstrainedTypeResult, ConstrainedTypeError};

/// A builder function for decimal values validating for min/max values
pub fn new_decimal<T, F>(
    field_name: &str,
    ctor: F,
    min_val: f64,
    max_val: f64,
    decimal: f64,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(f64) -> T
{
    if decimal < min_val {
        return ConstrainedTypeError::from(InvalidMinVal {
            field_name: field_name.to_string(),
            expected: min_val.to_string(),
            found: decimal.to_string(),
        }).into();
    }

    if decimal > max_val {
        return ConstrainedTypeError::from(InvalidMaxVal {
            field_name: field_name.to_string(),
            expected: max_val.to_string(),
            found: decimal.to_string(),
        }).into();
    }

    Ok(ctor(decimal))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};

    mod kilogram_quantity {
        use crate::error::ConstrainedTypeResult;
        use crate::decimal::new_decimal;

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct KilogramQuantity {
            value: f64
        }

        impl KilogramQuantity {
            pub(crate) const fn new(value: f64) -> Self {
                Self { value }
            }

            pub const fn value(&self) -> f64 { self.value }
        }

        pub fn new(
            field_name: &str,
            value: f64,
        ) -> ConstrainedTypeResult<KilogramQuantity> {
            new_decimal(
                field_name,
                |v| KilogramQuantity::new(v),
                0.05,
                100.0,
                value,
            )
        }
    }

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            kilogram_quantity::new(
                "qty",
                0.04,
            ),
            ConstrainedTypeError::from(InvalidMinVal {
                field_name: "qty".to_string(),
                expected: (0.05).to_string(),
                found: (0.04).to_string(),
            }).into()
        );

        assert_eq!(
            kilogram_quantity::new(
                "qty",
                100.1,
            ),
            ConstrainedTypeError::from(InvalidMaxVal {
                field_name: "qty".to_string(),
                expected: (100).to_string(),
                found: (100.1).to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_an_kilogram_quantity() {
        assert_eq!(
            kilogram_quantity::new(
                "qty",
                1.0,
            ).unwrap().value(),
            1.0
        );
    }
}
