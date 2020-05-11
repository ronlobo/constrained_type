//! Constrained String Option

#![deny(missing_docs)]

use crate::error::ConstrainedTypeErrorKind::InvalidMaxLen;
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining an optional String to not exceed a character limit
pub fn new_string_option<'a, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    val: Option<&'a str>,
) -> ConstrainedTypeResult<T>
where
    F: Fn(Option<&'a str>) -> T,
{
    if val != None && val.unwrap().len() > max_len {
        return ConstrainedTypeError::from(InvalidMaxLen {
            field_name: field_name.to_string(),
            expected: max_len.to_string(),
            found: val.unwrap().len().to_string(),
        })
        .into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::InvalidMaxLen;

    mod string_55_option {
        use crate::error::ConstrainedTypeResult;
        use crate::string_option::new_string_option;

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct String55Option {
            value: Option<String>,
        }

        impl String55Option {
            const MAX_LEN: usize = 55;

            pub(crate) fn new<S: Into<String>>(raw: Option<S>) -> String55Option {
                return match raw {
                    None => Self { value: None },
                    _ => Self {
                        value: Some(raw.unwrap().into()),
                    },
                };
            }

            pub fn value(&self) -> Option<&str> {
                return match self.value {
                    None => None,
                    _ => Some(self.value.as_ref().unwrap().as_str()),
                };
            }
        }

        pub fn new(field_name: &str, str: Option<&str>) -> ConstrainedTypeResult<String55Option> {
            new_string_option(
                field_name,
                |v| String55Option::new(v),
                String55Option::MAX_LEN,
                str,
            )
        }
    }

    #[test]
    fn it_validates_a_string55_option_max_len() {
        assert_eq!(
            string_55_option::new("name", Some("🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺"),),
            ConstrainedTypeError::from(InvalidMaxLen {
                field_name: "name".to_string(),
                expected: (55).to_string(),
                found: (56).to_string(),
            })
            .into()
        );
    }

    #[test]
    fn it_can_construct_a_string55option_with_some() {
        assert_eq!(
            string_55_option::new("name", Some("🐺"),).unwrap().value(),
            Some("🐺")
        );
    }

    #[test]
    fn it_can_construct_a_string55option_with_none() {
        assert_eq!(string_55_option::new("name", None,).unwrap().value(), None);
    }
}
