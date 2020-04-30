//! Error types for the crate

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};
use crate::error::{ConstrainedTypeResult, ConstrainedTypeError};

/// A builder function for integer values validating for min/max values
pub fn new_int<T, F>(
    field_name: &str,
    ctor: F,
    min_val: isize,
    max_val: isize,
    int: isize,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(isize) -> T
{
    if int < min_val {
        return Err(ConstrainedTypeError::from(InvalidMinVal {
            field_name: field_name.into(),
            expected: min_val.to_string(),
            found: int.to_string(),
        }));
    }

    if int > max_val {
        return Err(ConstrainedTypeError::from(InvalidMaxVal {
            field_name: field_name.into(),
            expected: max_val.to_string(),
            found: int.to_string(),
        }));
    }

    Ok(ctor(int))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMinVal, InvalidMaxVal};

    mod factor {
        use crate::int::new_int;
        use crate::error::ConstrainedTypeResult;

        const MIN_VAL: isize = -10;
        const MAX_VAL: isize = 10;

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Factor {
            pub(crate) value: isize
        }

        impl Factor {
            pub(crate) const fn new(value: isize) -> Self {
                Self { value }
            }

            pub const fn value(&self) -> isize { self.value }
        }

        pub fn new(
            field_name: &str,
            value: isize,
        ) -> ConstrainedTypeResult<Factor> {
            new_int(
                field_name,
                |v| Factor::new(v),
                MIN_VAL,
                MAX_VAL,
                value,
            )
        }
    }

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            factor::new(
                "factor",
                -11,
            ),
            ConstrainedTypeError::from(InvalidMinVal {
                field_name: "factor".to_string(),
                expected: (-10).to_string(),
                found: (-11).to_string(),
            }).into()
        );

        assert_eq!(
            factor::new(
                "factor",
                11,
            ),
            ConstrainedTypeError::from(InvalidMaxVal {
                field_name: "factor".to_string(),
                expected: 10.to_string(),
                found: 11.to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_a_factor() {
        assert_eq!(
            factor::new(
                "factor",
                -1,
            ).unwrap().value(),
            -1
        );
    }
}
