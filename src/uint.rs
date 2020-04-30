//! Error types for the crate

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};
use crate::error::{ConstrainedTypeResult, ConstrainedTypeError};

/// A builder function for unsigned integer values validating for min/max values
pub fn new_uint<T, F>(
    field_name: &str,
    ctor: F,
    min_val: usize,
    max_val: usize,
    uint: usize,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(usize) -> T
{
    if uint < min_val {
        return Err(ConstrainedTypeError::from(InvalidMinVal {
            field_name: field_name.into(),
            expected: min_val.to_string(),
            found: uint.to_string(),
        }));
    }

    if uint > max_val {
        return Err(ConstrainedTypeError::from(InvalidMaxVal {
            field_name: field_name.into(),
            expected: max_val.to_string(),
            found: uint.to_string(),
        }));
    }

    Ok(ctor(uint))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};

    mod unit_quantity {
        use crate::uint::new_uint;
        use crate::error::ConstrainedTypeResult;

        const MIN_VAL: usize = 1;
        const MAX_VAL: usize = 1000;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct UnitQuantity {
            pub(crate) value: usize
        }

        impl UnitQuantity {
            pub(crate) const fn new(value: usize) -> Self {
                Self { value }
            }

            pub const fn value(&self) -> usize { self.value }
        }

        pub fn new(
            field_name: &str,
            value: usize,
        ) -> ConstrainedTypeResult<UnitQuantity> {
            new_uint(
                field_name,
                |v| UnitQuantity::new(v),
                MIN_VAL,
                MAX_VAL,
                value,
            )
        }
    }

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            unit_quantity::new(
                "qty",
                0,
            ),
            ConstrainedTypeError::from(InvalidMinVal {
                field_name: "qty".to_string(),
                expected: 1.to_string(),
                found: 0.to_string(),
            }).into()
        );

        assert_eq!(
            unit_quantity::new(
                "qty",
                1001,
            ),
            ConstrainedTypeError::from(InvalidMaxVal {
                field_name: "qty".to_string(),
                expected: 1000.to_string(),
                found: 1001.to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_a_unit_quantity() {
        assert_eq!(
            unit_quantity::new(
                "qty",
                1,
            ).unwrap().value(),
            1
        );
    }
}
