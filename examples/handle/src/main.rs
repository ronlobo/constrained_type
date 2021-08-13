use crate::handle::{new};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let res = new("handle", &args.handle, None);

    match res {
        Ok(v) => { println!("{}", v.value()); }
        Err(_) => { println!("{:?}", res); }
    }
}

#[derive(StructOpt)]
struct Cli {
    pub handle: String,
}

///
/// # Example
///
/// Let's create a handle value object.
/// This reassembles the GCP service name, name, handle, etc.
///
/// # Pattern: ^[a-z][a-z0-9\-]{0,62}(?<!\-)[a-z0-9]{0,1}$
///
/// # Dissecting the pattern:
///
/// ^                                            Match the beginning of the string
/// [a-z]                                        Require that the string starts with a lower-case letter
/// [a-z0-9\-]{0,62}                             Optionally match a lower-case letter, number or hyphen up to 62 times
/// (?<!\-)                                      Negative lookbehind to make sure the last character cannot be a hyphen
/// [a-z0-9]{0,1}                                Require that the last character is a lower-case letter or number
/// $                                            Match the end of the string.
///
mod handle {
    use constrained_type::error::ConstrainedTypeResult;
    use constrained_type::string_like::new_string_like;
    use fancy_regex::Regex;

    pub const HANDLE_PATTERN: &str = r"^[a-z][a-z0-9\-]{0,62}(?<!\-)[a-z0-9]{0,1}$";

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Handle(pub(crate) String);

    impl Handle {
        pub(crate) fn new<S: Into<String>>(raw: S) -> Handle {
            Self(raw.into())
        }

        pub fn value(&self) -> &str {
            &self.0
        }
    }

    impl std::fmt::Display for Handle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.value())
        }
    }

    pub fn new(field_name: &str, raw: &str, err_val: Option<&str>) -> ConstrainedTypeResult<Handle> {
        new_string_like(
            field_name,
            |v| Handle::new(v),
            Regex::new(HANDLE_PATTERN).unwrap(),
            raw,
            err_val,
        )
    }
}

#[cfg(test)]
pub mod test {
    use crate::handle::{new, HANDLE_PATTERN, Handle};
    use constrained_type::error::ConstrainedTypeErrorKind::InvalidPattern;
    use constrained_type::error::ConstrainedTypeError;

    #[test]
    fn test_handle() {
        assert_eq!(
            new("handle", "1", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: "1".to_string(),
            }).into()
        );

        assert_eq!(
            new("handle", "1a", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: "1a".to_string(),
            }).into()
        );

        assert_eq!(
            new("handle", "-a", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: "-a".to_string(),
            }).into()
        );

        assert_eq!(
            new("handle", "a-", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: "a-".to_string(),
            }).into()
        );

        assert_eq!(
            new("handle", "A", None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: "A".to_string(),
            }).into()
        );

        let mut handle = "aaaaaaaa".repeat(8);
        handle.push_str("a");

        assert_eq!(
            new("handle", &handle, None),
            ConstrainedTypeError::from(InvalidPattern {
                field_name: "handle".to_string(),
                expected: HANDLE_PATTERN.to_string(),
                found: handle,
            }).into()
        );

        assert_eq!(
            new("handle", "a", None),
            Ok(Handle("a".to_string()))
        );

        assert_eq!(
            new("handle", "a1", None),
            Ok(Handle("a1".to_string()))
        );

        assert_eq!(
            new("handle", "a-1", None),
            Ok(Handle("a-1".to_string()))
        );

        let handle = "aaaaaaaa".repeat(8);

        assert_eq!(
            new("handle", &handle, None),
            Ok(Handle(handle.to_string()))
        );

        assert_eq!(format!("{:?}", new("handle", "a-1", None).unwrap()), "Handle(\"a-1\")");

        assert_eq!(format!("{}", new("handle", "a-1", None).unwrap()), "a-1");
    }
}
