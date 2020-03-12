use crate::core::{CtorResult, new_string_like};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmailAddress<'a> {
    str: &'a str
}

impl<'a> EmailAddress<'a> {
    pub(crate) const fn new(str: &'a str) -> Self { Self { str } }

    pub const fn value(&self) -> &'a str { self.str }
}

pub fn new<'a>(field_name: &str, str: &'a str) -> CtorResult<EmailAddress<'a>> {
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
            Ok(Some(EmailAddress::new("acmeinc@example.com")))
        );
    }
}
