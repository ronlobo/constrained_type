//! Constrained integer number

#![deny(missing_docs)]

use num_traits::PrimInt;

use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};
use crate::error::ConstrainedTypeErrorKind::{InvalidMaxVal, InvalidMinVal};

/// A builder function constraining an integer number between a minimum and maximum value
pub fn new_int<T, F, V>(
    field_name: &str,
    ctor: F,
    min_val: V,
    max_val: V,
    val: V,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(V) -> T,
        V: PrimInt + ToString
{
    if val < min_val {
        return ConstrainedTypeError::from(InvalidMinVal {
            field_name: field_name.into(),
            expected: min_val.to_string(),
            found: val.to_string(),
        }).into();
    }

    if val > max_val {
        return ConstrainedTypeError::from(InvalidMaxVal {
            field_name: field_name.into(),
            expected: max_val.to_string(),
            found: val.to_string(),
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMaxVal, InvalidMinVal};

    mod unit_quantity {
        use crate::error::ConstrainedTypeResult;
        use crate::int::new_int;

        const MIN_VAL: isize = 1;
        const MAX_VAL: isize = 1000;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct UnitQuantity {
            pub(crate) value: isize
        }

        impl UnitQuantity {
            pub(crate) const fn new(value: isize) -> Self {
                Self { value }
            }

            pub const fn value(&self) -> isize { self.value }
        }

        pub fn new(
            field_name: &str,
            value: isize,
        ) -> ConstrainedTypeResult<UnitQuantity> {
            new_int(
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
