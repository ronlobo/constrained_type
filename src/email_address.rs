use crate::core::{CtorResult, new_string_like};

pub type EmailAddressCtorResult<'value> = CtorResult<EmailAddress<'value>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmailAddress<'a> {
    value: &'a str
}

impl<'a> EmailAddress<'a> {
    pub(crate) const fn new(value: &'a str) -> Self { Self { value } }

    pub const fn value(&self) -> &'a str { self.value }
}

pub fn new<'field_name, 'value>(
    field_name: &'field_name str,
    value: &'value str,
) -> EmailAddressCtorResult<'value> {
    new_string_like::<EmailAddress<'value>>(
        field_name,
        &EmailAddress::new,
        r".+@.+",
        value,
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
