use crate::email_address::{new};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let res = new("email-address", &args.email, None);

    match res {
        Ok(v) => { println!("{}", v.value()); }
        Err(_) => { println!("{:?}", res); }
    }
}

#[derive(StructOpt)]
struct Cli {
    pub email: String,
}

mod email_address {
    use constrained_type::error::ConstrainedTypeResult;
    use constrained_type::string_like::new_string_like;
    use fancy_regex::Regex;

    pub(crate) const EMAIL_PATTERN: &str = r".+@.+";

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct EmailAddress(pub(crate) String);

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

#[cfg(test)]
pub mod test {
    use crate::email_address::{new, EMAIL_PATTERN, EmailAddress};
    use constrained_type::error::ConstrainedTypeErrorKind::InvalidPattern;
    use constrained_type::error::ConstrainedTypeError;

    #[test]
    fn it_errors_on_invalid_email_address() {
        assert_eq!(
            new("email-address", "@something", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email-address".to_string(),
                expected: EMAIL_PATTERN.to_string(),
                found: "@something".to_string(),
            }).into()
        );

        assert_eq!(
            new("email-address", "", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email-address".to_string(),
                expected: EMAIL_PATTERN.to_string(),
                found: "".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_redacts_value_with_error_value() {
        assert_eq!(
            new("email-address", "@something", Some("<redacted>")),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "email-address".to_string(),
                expected: EMAIL_PATTERN.to_string(),
                found: "<redacted>".to_string(),
            }).into()
        );
    }

    #[test]
    fn it_can_construct_an_email_address() {
        assert_eq!(
            new("email-address", "acmeinc@example.com", None),
            Ok(EmailAddress("acmeinc@example.com".to_string()))
        );
    }
}
