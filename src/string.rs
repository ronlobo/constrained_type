//! Constrained String

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::{InvalidMaxLen, InvalidOption};
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining a String to be not empty and neither exceeding a character limit
pub fn new_string<'a, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    val: &'a str,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(&'a str) -> T,
{
    if val.is_empty() {
        return ConstrainedTypeError::from(InvalidOption {
            field_name: field_name.to_string(),
        }).into();
    }

    if val.chars().count() > max_len {
        return ConstrainedTypeError::from(InvalidMaxLen {
            field_name: field_name.to_string(),
            expected: max_len.to_string(),
            found: val.chars().count().to_string(),
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::{InvalidMaxLen, InvalidOption};

    mod string_5 {
        use crate::error::ConstrainedTypeResult;
        use crate::string::new_string;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct String5 {
            value: String,
        }

        impl String5 {
            const MAX_LEN: usize = 5;

            pub(crate) fn new<S: Into<String>>(raw: S) -> String5 {
                Self { value: raw.into() }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        pub fn new(field_name: &str, str: &str) -> ConstrainedTypeResult<String5> {
            new_string(field_name, |v| String5::new(v), String5::MAX_LEN, str)
        }
    }

    #[test]
    fn it_validates_an_empty_string5() {
        assert_eq!(
            string_5::new("name", ""),
            ConstrainedTypeError::from(InvalidOption {
                field_name: "name".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_validates_a_string5_max_len() {
        assert_eq!(
            string_5::new("name", "🐺🐺🐺🐺🐺🐺"),
            ConstrainedTypeError::from(InvalidMaxLen {
                field_name: "name".to_string(),
                expected: (5).to_string(),
                found: (6).to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_a_string5() {
        assert_eq!(string_5::new("name", "🐺").unwrap().value(), "🐺");
    }
}
