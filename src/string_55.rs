use crate::core::{CtorResult, new_string, new_string_option};

pub type String55Option<'a> = Option<String55<'a>>;

pub type String55CtorResult<'value> = CtorResult<String55<'value>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct String55<'a> {
    value: &'a str
}

impl<'a> String55<'a> {
    pub(crate) const fn new(value: &'a str) -> Self { Self { value } }

    pub const fn value(&self) -> &'a str { self.value }
}

pub fn new<'field_name, 'value>(
    field_name: &'field_name str,
    str: &'value str,
) -> String55CtorResult<'value> {
    new_string::<String55<'value>>(
        field_name,
        &String55::new,
        55,
        str,
    )
}

pub fn new_option<'field_name, 'value>(
    field_name: &'field_name str,
    str: Option<&'value str>,
) -> String55CtorResult<'value> {
    new_string_option::<String55<'value>>(
        field_name,
        &String55::new,
        55,
        str,
    )
}

#[cfg(test)]
pub mod test {
    use crate::string_55::String55;

    #[test]
    fn it_validates_an_empty_string55() {
        assert_eq!(
            crate::string_55::new("chars", ""),
            Err("chars must not be empty".to_string())
        );
    }

    #[test]
    fn it_validates_a_string55_max_len() {
        assert_eq!(
            crate::string_55::new(
                "chars",
                "🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺🐺",
            ),
            Err("chars must not be greater than 55 characters".to_string())
        );
    }

    #[test]
    fn it_can_construct_a_string55() {
        assert_eq!(
            crate::string_55::new(
                "chars",
                "🐺",
            )
                .unwrap()
                .unwrap()
                .value(),
            "🐺"
        );
    }

    #[test]
    fn it_can_construct_a_string55option_with_some() {
        assert_eq!(
            crate::string_55::new_option(
                "chars",
                Some("🐺"),
            ),
            Ok(Some(String55::new("🐺")))
        );
    }

    #[test]
    fn it_can_construct_a_string55option_with_none() {
        assert_eq!(
            crate::string_55::new_option(
                "chars",
                None,
            ),
            Ok(None)
        );
    }
}
