use crate::core::{CtorResult, new_decimal};

type KilogramQuantityCtorResult = CtorResult<KilogramQuantity>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct KilogramQuantity {
    value: f64
}

impl KilogramQuantity {
    pub(crate) const fn new(value: f64) -> Self { Self { value } }

    pub const fn value(&self) -> f64 { self.value }
}

pub fn new(
    field_name: &str,
    value: f64,
) -> KilogramQuantityCtorResult {
    new_decimal(
        field_name,
        |v| KilogramQuantity::new(v),
        0.05,
        100.0,
        value,
    )
}

#[cfg(test)]
pub mod test {
    use crate::kilogram_quantity::KilogramQuantity;

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            crate::kilogram_quantity::new(
                "qty",
                0.04,
            ),
            Err("qty must not be less than 0.05".to_string())
        );

        assert_eq!(
            crate::kilogram_quantity::new(
                "qty",
                100.1,
            ),
            Err("qty must not be greater than 100".to_string())
        );
    }

    #[test]
    fn it_can_construct_an_kilogram_quantity() {
        assert_eq!(
            crate::kilogram_quantity::new(
                "qty",
                1.0,
            ),
            Ok(Some(KilogramQuantity::new(1.0)))
        );
    }
}
