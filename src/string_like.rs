//! Constrained String Like

#![deny(missing_docs)]

use regex::Regex;

use crate::error::ConstrainedTypeErrorKind::InvalidPattern;
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining a String to match a given pattern
pub fn new_string_like<'a, T, F>(
    field_name: &str,
    ctor: F,
    pattern: Regex,
    val: &'a str,
) -> ConstrainedTypeResult<T>
    where
        F: Fn(&'a str) -> T,
{
    if !pattern.is_match(val) {
        return ConstrainedTypeError::from(InvalidPattern {
            field_name: field_name.to_string(),
            expected: pattern.to_string(),
            found: val.to_string(),
        }).into();
    }

    Ok(ctor(val))
}

#[cfg(test)]
pub mod test {
    use crate::error::ConstrainedTypeError;
    use crate::error::ConstrainedTypeErrorKind::InvalidPattern;

    mod email_address {
        use regex::Regex;

        use crate::error::ConstrainedTypeResult;
        use crate::string_like::new_string_like;

        const EMAIL_PATTERN: &str = r".+@.+";

        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub struct EmailAddress {
            value: String,
        }

        impl EmailAddress {
            pub(crate) fn new<S: Into<String>>(raw: S) -> EmailAddress {
                Self { value: raw.into() }
            }

            pub fn value(&self) -> &str {
                &self.value
            }
        }

        pub fn new(field_name: &str, raw: &str) -> ConstrainedTypeResult<EmailAddress> {
            new_string_like(
                field_name,
                |v| EmailAddress::new(v),
                Regex::new(EMAIL_PATTERN).unwrap(),
                raw,
            )
        }
    }

    #[test]
    fn it_errors_on_invalid_email_address() {
        assert_eq!(
            email_address::new("email", "@something"),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email".to_string(),
                expected: r".+@.+".to_string(),
                found: "@something".to_string(),
            }).into()
        );

        assert_eq!(
            email_address::new("email", ""),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email".to_string(),
                expected: r".+@.+".to_string(),
                found: "".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_an_email_address() {
        assert_eq!(
            email_address::new("email", "acmeinc@example.com")
                .unwrap()
                .value(),
            "acmeinc@example.com"
        );
    }
}
