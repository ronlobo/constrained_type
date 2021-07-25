//! Constrained String Like

#![deny(missing_docs)]

use fancy_regex::Regex;

use crate::error::ConstrainedTypeErrorKind::InvalidPattern;
use crate::error::{ConstrainedTypeError, ConstrainedTypeResult};

/// A builder function constraining a String to match a given pattern
///
/// # Example
///
/// Let's create a password value object.
///
/// As additional security measure, we want to make sure the password can only be read once.
/// You can read more about this in the book "Secure by design".
///
/// We apply a password pattern described at:
/// https://www.ocpsoft.org/tutorials/regular-expressions/password-regular-expression/
///
/// Additionally we escape the special characters "^&[]\/" as they can have special meaning and might appear in character classes.
///
/// # Pattern: ^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*[*.!@$%\^\&(){}\[\]:;<>,.?\/~_+-=|\\]).{8,32}$
///
/// # Dissecting the pattern:
///
/// ^                                            Match the beginning of the string
/// (?=.*[0-9])                                  Require that at least one digit appear anywhere in the string
/// (?=.*[a-z])                                  Require that at least one lowercase letter appear anywhere in the string
/// (?=.*[A-Z])                                  Require that at least one uppercase letter appear anywhere in the string
/// (?=.*[*.!@$%\^\&(){}\[\]:;<>,.?\/~_+-=|\\])  Require that at least one special character appear anywhere in the string
/// .{8,32}                                      The password must be at least 8 characters long, but no more than 32
/// $                                            Match the end of the string.
///
/// ```rust
///
/// // mod password {
///
///     use constrained_type::error;
///     use constrained_type::error::ConstrainedTypeResult;
///     use constrained_type::error::ConstrainedTypeError;
///     use constrained_type::error::ConstrainedTypeErrorKind::InvalidPattern;
///     use constrained_type::string_like::new_string_like;
///     use fancy_regex::Regex;
///
///     pub const PASSWORD_PATTERN: &str = r"^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*[*.!@$%\^\&(){}\[\]:;<>,.?\/~_+\-=|\\]).{8,32}$";
///
///     #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
///     pub struct Password {
///         value: Option<String>,
///      }
///
///     impl Password {
///         pub(crate) fn new<S: Into<String>>(raw: S) -> Password {
///                 Self { value: Some(raw.into()) }
///         }
///
///         pub fn value(&mut self) -> Result<String, &str> {
///             match &self.value {
///                 Some(_) => {
///                     let value = self.value.as_ref().unwrap().to_string();
///                     self.value = None;
///
///                     Ok(value)
///                 }
///                None => Err("Password has already be consumed.")
///             }
///         }
///     }
///
///     pub fn new(field_name: &str, raw: &str, err_val: Option<&str>) -> ConstrainedTypeResult<Password> {
///         new_string_like(
///             field_name,
///             |v| Password::new(v),
///             Regex::new(PASSWORD_PATTERN).unwrap(),
///             raw,
///             err_val,
///         )
///     }
///
///     assert_eq!(
///         new("password", "mypass", Some("<redacted>")),
///         ConstrainedTypeError::from(InvalidPattern {
///                     field_name: "password".to_string(),
///                     expected: PASSWORD_PATTERN.to_string(),
///                     found: "<redacted>".to_string(),
///                 }).into()
///     );
///
///     assert_eq!(
///         new("password", "T3st^^^^", Some("<redacted>")),
///         Ok(Password { value: Some("T3st^^^^".to_string()) })
///     );
///
///     assert_eq!(
///         new("password", "T3st&&&&", Some("<redacted>")),
///         Ok(Password { value: Some("T3st&&&&".to_string()) })
///     );
///
///     assert_eq!(
///         new("password", "T3st[[[[", Some("<redacted>")),
///         Ok(Password { value: Some("T3st[[[[".to_string()) })
///     );
///
///     assert_eq!(
///         new("password", "T3st]]]]", Some("<redacted>")),
///         Ok(Password { value: Some("T3st]]]]".to_string()) })
///     );
///
///     assert_eq!(
///         new("password", "T3st////", Some("<redacted>")),
///         Ok(Password { value: Some("T3st////".to_string()) })
///     );
///
///     assert_eq!(
///         new("password", "T3st\\\\\\\\", Some("<redacted>")),
///         Ok(Password { value: Some("T3st\\\\\\\\".to_string()) })
///     );
///
///     let mut pass = new("password", "T3st?^&[]", Some("<redacted>")).unwrap();
///
///     let val = pass.value();
///
///     assert_eq!(
///         val,
///         Ok("T3st?^&[]".to_string())
///     );
///
///     let val = pass.value();
///
///     assert_eq!(
///         val,
///         Err("Password has already be consumed.")
///     );
///
/// // }
///
/// ```
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
