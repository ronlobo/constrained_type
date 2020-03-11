use regex::Regex;

pub type StringCtor<'a, T> = dyn Fn(&'a str) -> T;
pub type IntCtor<T> = dyn Fn(isize) -> T;
pub type UIntCtor<T> = dyn Fn(usize) -> T;
pub type DecimalCtor<T> = dyn Fn(f64) -> T;
pub type CtorResult<T> = Result<Option<T>, String>;

pub fn new_string<'value, 'raw, T>(
    field_name: &'value str,
    ctor: &'value StringCtor<'raw, T>,
    max_len: usize,
    raw: &'raw str,
) -> CtorResult<T> {
    if raw.len() == 0 {
        return Err(format!("{} must not be empty", field_name.to_string()));
    }

    if raw.len() > max_len {
        return Err(format!("{} must not be greater than {} characters", field_name.to_string(), max_len));
    }

    Ok(Option::from(ctor(raw)))
}

pub fn new_string_option<'value, 'raw, T>(
    field_name: &'value str,
    ctor: &'value StringCtor<'raw, T>,
    max_len: usize,
    str: Option<&'raw str>,
) -> CtorResult<T> {
    if str == None {
        return Ok(None);
    }

    if str.unwrap().len() > max_len {
        return Err(format!("{} must not be greater than {} characters", field_name.to_string(), max_len));
    }

    Ok(Some(ctor(str.unwrap())))
}

pub fn new_string_like<'field_name, 'value, 'regex, T>(
    field_name: &'field_name str,
    ctor: &'value StringCtor<'value, T>,
    pattern: &'regex str,
    str: &'value str,
) -> CtorResult<T> {
    if Regex::new(pattern).unwrap().is_match(str) {
        return Ok(Option::from(ctor(str)));
    }

    Err(format!(
        "'{}': '{}' must match the pattern '{}'",
        field_name.to_string(),
        str.to_string(),
        pattern.to_string()
    ))
}

pub fn new_int<'field_name, T>(
    field_name: &'field_name str,
    ctor: &IntCtor<T>,
    min_val: isize,
    max_val: isize,
    int: isize,
) -> CtorResult<T> {
    if int < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if int > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(int)))
}

pub fn new_uint<'field_name, T>(
    field_name: &'field_name str,
    ctor: &UIntCtor<T>,
    min_val: usize,
    max_val: usize,
    int: usize,
) -> CtorResult<T> {
    if int < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if int > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(int)))
}

pub fn new_decimal<'field_name, T>(
    field_name: &'field_name str,
    ctor: &DecimalCtor<T>,
    min_val: f64,
    max_val: f64,
    decimal: f64,
) -> CtorResult<T> {
    if decimal < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if decimal > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(decimal)))
}
