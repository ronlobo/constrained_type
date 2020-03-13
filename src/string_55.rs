use crate::core::{CtorResult, new_string, new_string_option};

pub type String55Option = Option<String55>;

pub type String55CtorResult = CtorResult<String55>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct String55 {
    value: String
}

impl String55 {
    pub(crate) fn new<S: Into<String>>(raw: S) -> CtorResult<String55> {
        Ok(Some(Self { value: raw.into() }))
    }

    pub fn value(&self) -> &str { &self.value }
}

pub fn new(
    field_name: &str,
    str: &str,
) -> String55CtorResult {
    new_string(
        field_name,
        |v| String55::new(v),
        55,
        str,
    )
}

pub fn new_option(
    field_name: &str,
    str: Option<&str>,
) -> String55CtorResult {
    new_string_option(
        field_name,
        |v| String55::new(v),
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
            String55::new("🐺")
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
