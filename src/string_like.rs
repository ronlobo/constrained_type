//! Constrained String Like

#![deny(missing_docs)]

use fancy_regex::Regex;

use crate::error::ConstrainedTypeErrorKind::InvalidPattern;
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining a String to match a given pattern
pub fn new_string_like<'val, 'err_val, T, F>(
    field_name: &str,
    ctor: F,
    pattern: Regex,
    val: &'val str,
    err_val: Option<&'err_val str>,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(&'val str) -> T,
{
    if !pattern.is_match(val).unwrap() {
        return ConstrainedTypeError::from(InvalidPattern {
            field_name: field_name.to_string(),
            expected: pattern.to_string(),
            found: {
                if err_val != None {
                    err_val.unwrap().to_string()
                } else {
                    val.to_string()
                }
            },
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
pub mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::InvalidPattern;

    mod email_address {
        use crate::error::ConstrainedTypeResult;
        use crate::string_like::new_string_like;
        use fancy_regex::Regex;

        const EMAIL_PATTERN: &str = r".+@.+";

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct EmailAddress(String);

        impl EmailAddress {
            pub(crate) fn new<S: Into<String>>(raw: S) -> EmailAddress {
                Self(raw.into())
            }

            pub fn value(&self) -> &str {
                &self.0
            }
        }

        pub fn new(field_name: &str, raw: &str, err_val: Option<&str>) -> ConstrainedTypeResult<EmailAddress> {
            new_string_like(
                field_name,
                |v| EmailAddress::new(v),
                Regex::new(EMAIL_PATTERN).unwrap(),
                raw,
                err_val,
            )
        }
    }

    #[test]
    fn it_errors_on_invalid_email_address() {
        assert_eq!(
            email_address::new("email", "@something", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email".to_string(),
                expected: r".+@.+".to_string(),
                found: "@something".to_string(),
            }).into()
        );

        assert_eq!(
            email_address::new("email", "", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email".to_string(),
                expected: r".+@.+".to_string(),
                found: "".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_redacts_value_with_error_value() {
        assert_eq!(
            email_address::new("email", "@something", Some("<redacted>")),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email".to_string(),
                expected: r".+@.+".to_string(),
                found: "<redacted>".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_an_email_address() {
        assert_eq!(
            email_address::new("email", "acmeinc@example.com", None)
                .unwrap()
                .value(),
            "acmeinc@example.com"
        );
    }
}
