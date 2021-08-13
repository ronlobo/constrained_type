use crate::password::{new};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let res = new("password", &args.password, None);

    match res {
        Ok(mut p) => { println!("{}", p.value().unwrap()); }
        Err(_) => { println!("{:?}", res); }
    }
}

#[derive(StructOpt)]
struct Cli {
    pub password: String,
}

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
mod password {
    use constrained_type::error::ConstrainedTypeResult;
    use constrained_type::string_like::new_string_like;
    use fancy_regex::Regex;
    use zeroize::Zeroize;

    pub const PASSWORD_PATTERN: &str = r"^(?=.*[0-9])(?=.*[a-z])(?=.*[A-Z])(?=.*[*.!@$%\^\&(){}\[\]:;<>,.?\/~_+\-=|\\]).{8,32}$";

    #[derive(Clone, PartialEq, Eq)]
    pub struct Password(pub(crate) Option<String>);

    impl Password {
        pub(crate) fn new<S: Into<String>>(raw: S) -> Password {
            Self(Some(raw.into()))
        }

        pub fn value(&mut self) -> Result<String, &str> {
            match &self.0 {
                Some(_) => {
                    let value = self.0.as_ref().unwrap().to_string();
                    self.0 = None;

                    Ok(value)
                }
                None => Err("Password has already be consumed.")
            }
        }
    }

    impl Drop for Password {
        fn drop(&mut self) {
            let v = &mut self.0;

            match v {
                Some(_) => {
                    v.zeroize();
                }
                _ => {}
            }
        }
    }

    impl std::fmt::Debug for Password {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "\"*******\"")
        }
    }

    impl std::fmt::Display for Password {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "*******")
        }
    }

    pub fn new(field_name: &str, raw: &str, err_val: Option<&str>) -> ConstrainedTypeResult<Password> {
        new_string_like(
            field_name,
            |v| Password::new(v),
            Regex::new(PASSWORD_PATTERN).unwrap(),
            raw,
            err_val,
        )
    }
}

#[cfg(test)]
pub mod test {
    use crate::password::{new, PASSWORD_PATTERN, Password};
    use constrained_type::error::ConstrainedTypeErrorKind::InvalidPattern;
    use constrained_type::error::ConstrainedTypeError;

    #[test]
    fn test_password() {
        assert_eq!(
            new("password", "mypass", Some("<redacted>")),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "password".to_string(),
                expected: PASSWORD_PATTERN.to_string(),
                found: "<redacted>".to_string(),
            }).into()
        );

        assert_eq!(
            new("password", "T3st^^^^", Some("<redacted>")),
            Ok(Password(Some("T3st^^^^".to_string())))
        );

        assert_eq!(
            new("password", "T3st&&&&", Some("<redacted>")),
            Ok(Password(Some("T3st&&&&".to_string())))
        );

        assert_eq!(
            new("password", "T3st[[[[", Some("<redacted>")),
            Ok(Password(Some("T3st[[[[".to_string())))
        );

        assert_eq!(
            new("password", "T3st]]]]", Some("<redacted>")),
            Ok(Password(Some("T3st]]]]".to_string())))
        );

        assert_eq!(
            new("password", "T3st////", Some("<redacted>")),
            Ok(Password(Some("T3st////".to_string())))
        );

        assert_eq!(
            new("password", "T3st\\\\\\\\", Some("<redacted>")),
            Ok(Password(Some("T3st\\\\\\\\".to_string())))
        );

        let mut pass = new("password", "T3st?^&[]", Some("<redacted>")).unwrap();

        let val = pass.value();

        assert_eq!(
            val,
            Ok("T3st?^&[]".to_string())
        );

        let val = pass.value();

        assert_eq!(
            val,
            Err("Password has already be consumed.")
        );

        assert_eq!(format!("{:?}", new("password", "T3st?^&[]", Some("<redacted>")).unwrap()), "\"*******\"");

        assert_eq!(format!("{}", new("password", "T3st?^&[]", Some("<redacted>")).unwrap()), "*******");
    }
}