use crate::core::{CtorResult, new_uint};

type UnitQuantityCtorResult = CtorResult<UnitQuantity>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitQuantity {
    value: usize
}

impl UnitQuantity {
    pub(crate) const fn new(value: usize) -> Self { Self { value } }

    pub const fn value(&self) -> usize { self.value }
}

pub fn new(
    field_name: &str,
    value: usize,
) -> UnitQuantityCtorResult {
    new_uint::<UnitQuantity>(
        field_name,
        &UnitQuantity::new,
        1,
        1000,
        value,
    )
}

#[cfg(test)]
pub mod test {
    use crate::unit_quantity::UnitQuantity;

    #[test]
    fn it_errors_on_out_of_bounds_value() {
        assert_eq!(
            crate::unit_quantity::new(
                "qty",
                0,
            ),
            Err("qty must not be less than 1".to_string())
        );

        assert_eq!(
            crate::unit_quantity::new(
                "qty",
                1001,
            ),
            Err("qty must not be greater than 1000".to_string())
        );
    }

    #[test]
    fn it_can_construct_a_unit_quantity() {
        assert_eq!(
            crate::unit_quantity::new(
                "qty",
                1,
            ),
            Ok(Some(UnitQuantity::new(1)))
        );
    }
}
