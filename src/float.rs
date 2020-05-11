//! Constrained floating point number

#![deny(missing_docs)]

use num_traits::Float;

use crate::error::ConstrainedTypeErrorKind::{InvalidMaxVal, InvalidMinVal};
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining a floating point number between a min/max value
pub fn new_float<T, F, V>(
    field_name: &str,
    ctor: F,
    min_val: V,
    max_val: V,
    val: V,
) -> ConstrainedTypeResult<T>
where
    F: Fn(V) -> T,
    V: Float + ToString,
{
    if val < min_val {
        return ConstrainedTypeError::from(InvalidMinVal {
            field_name: field_name.to_string(),
            expected: min_val.to_string(),
            found: val.to_string(),
        })
        .into();
    }

    if val > max_val {
        return ConstrainedTypeError::from(InvalidMaxVal {
            field_name: field_name.to_string(),
            expected: max_val.to_string(),
            found: val.to_string(),
        })
        .into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMaxVal, InvalidMinVal};

    mod kilogram_quantity {
        use crate::error::ConstrainedTypeResult;
        use crate::float::new_float;

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct KilogramQuantity {
            value: f64,
        }

        impl KilogramQuantity {
            pub(crate) const fn new(value: f64) -> Self {
                Self { value }
            }

            pub const fn value(&self) -> f64 {
                self.value
            }
        }

        pub fn new(field_name: &str, value: f64) -> ConstrainedTypeResult<KilogramQuantity> {
            new_float(field_name, |v| KilogramQuantity::new(v), 0.05, 100.0, value)
        }
    }

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            kilogram_quantity::new("qty", 0.04),
            ConstrainedTypeError::from(InvalidMinVal {
                field_name: "qty".to_string(),
                expected: (0.05).to_string(),
                found: (0.04).to_string(),
            })
            .into()
        );

        assert_eq!(
            kilogram_quantity::new("qty", 100.1),
            ConstrainedTypeError::from(InvalidMaxVal {
                field_name: "qty".to_string(),
                expected: (100).to_string(),
                found: (100.1).to_string(),
            })
            .into()
        );
    }

    #[test]
    fn it_can_construct_an_kilogram_quantity() {
        assert_eq!(kilogram_quantity::new("qty", 1.0).unwrap().value(), 1.0);
    }
}
