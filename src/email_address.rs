use crate::core::{CtorResult, new_string_like};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmailAddress {
    value: String
}

impl EmailAddress {
    pub(crate) fn new<S: Into<String>>(raw: S) -> CtorResult<EmailAddress> {
        Ok(Some(Self { value: raw.into() }))
    }

    pub fn value(&self) -> &str { self.value.as_str() }
}

pub fn new(field_name: &str, str: &str) -> CtorResult<EmailAddress> {
    new_string_like(
        field_name,
        |v| EmailAddress::new(v),
        r".+@.+",
        str,
    )
}

#[cfg(test)]
pub mod test {
    use crate::email_address::EmailAddress;

    #[test]
    fn it_errors_on_invalid_email_address() {
        assert_eq!(
            crate::email_address::new(
                "email",
                "@something",
            ),
            Err("'email': '@something' must match the pattern '.+@.+'".to_string())
        );

        assert_eq!(
            crate::email_address::new(
                "email",
                "",
            ),
            Err("'email': '' must match the pattern '.+@.+'".to_string()));
    }

    #[test]
    fn it_can_construct_an_email_address() {
        assert_eq!(
            crate::email_address::new(
                "email",
                "acmeinc@example.com",
            ),
            EmailAddress::new("acmeinc@example.com")
        );
    }
}
