//! Constrained String

#![deny(missing_docs)]

use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};
use crate::error::ConstrainedTypeErrorKind::{InvalidMaxLen, InvalidOption};

/// A builder function constraining a String to be not empty and neither exceeding a character limit
pub fn new_string<'a, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    val: &'a str,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(&'a str) -> T
{
    if val.len() == 0 {
        return ConstrainedTypeError::from(InvalidOption {
            field_name: field_name.to_string()
        }).into();
    }

    if val.len() > max_len {
        return ConstrainedTypeError::from(InvalidMaxLen {
            field_name: field_name.to_string(),
            expected: max_len.to_string(),
            found: val.len().to_string(),
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMaxLen, InvalidOption};

    mod string_55 {
        use crate::error::ConstrainedTypeResult;
        use crate::string::new_string;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct String55 {
            value: String
        }

        impl String55 {
            const MAX_LEN: usize = 55;

            pub(crate) fn new<S: Into<String>>(raw: S) -> String55 {
                Self { value: raw.into() }
            }

            pub fn value(&self) -> &str { &self.value }
        }

        pub fn new(
            field_name: &str,
            str: &str,
        ) -> ConstrainedTypeResult<String55> {
            new_string(
                field_name,
                |v| String55::new(v),
                String55::MAX_LEN,
                str,
            )
        }
    }

    #[test]
    fn it_validates_an_empty_string55() {
        assert_eq!(
            string_55::new("name", ""),
            ConstrainedTypeError::from(InvalidOption {
                field_name: "name".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_validates_a_string55_max_len() {
        assert_eq!(
            string_55::new(
                "name",
                "🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺",
            ),
            ConstrainedTypeError::from(InvalidMaxLen {
                field_name: "name".to_string(),
                expected: (55).to_string(),
                found: (56).to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_a_string55() {
        assert_eq!(
            string_55::new(
                "name",
                "🐺",
            ).unwrap().value(),
            "🐺"
        );
    }
}
