//! Constrained String Option

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::InvalidMaxLen;
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining an optional String to not exceed a character limit
pub fn new_string_option<'val, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    val: Option<&'val str>,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(Option<&'val str>) -> T,
{
    if val != None && val.unwrap().chars().count() > max_len {
        return ConstrainedTypeError::from(InvalidMaxLen {
            field_name: field_name.to_string(),
            expected: max_len.to_string(),
            found: val.unwrap().chars().count().to_string(),
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::InvalidMaxLen;

    mod string_5_option {
        use crate::error::ConstrainedTypeResult;
        use crate::string_option::new_string_option;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct String5Option(Option<String>);

        impl String5Option {
            const MAX_LEN: usize = 5;

            pub(crate) fn new<S: Into<String>>(raw: Option<S>) -> String5Option {
                return match raw {
                    None => Self(None),
                    _ => Self(Some(raw.unwrap().into()))
                };
            }

            pub fn value(&self) -> Option<&str> {
                return match self.0 {
                    None => None,
                    _ => Some(self.0.as_ref().unwrap().as_str()),
                };
            }
        }

        pub fn new(field_name: &str, str: Option<&str>) -> ConstrainedTypeResult<String5Option> {
            new_string_option(
                field_name,
                |v| String5Option::new(v),
                String5Option::MAX_LEN,
                str,
            )
        }
    }

    #[test]
    fn it_validates_a_string5_option_max_len() {
        assert_eq!(
            string_5_option::new("name", Some("🐺🐺🐺🐺🐺🐺")),
            ConstrainedTypeError::from(InvalidMaxLen {
                field_name: "name".to_string(),
                expected: (5).to_string(),
                found: (6).to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_a_string5option_with_some() {
        assert_eq!(
            string_5_option::new("name", Some("🐺")).unwrap().value(),
            Some("🐺")
        );
    }

    #[test]
    fn it_can_construct_a_string55option_with_none() {
        assert_eq!(string_5_option::new("name", None).unwrap().value(), None);
    }
}
