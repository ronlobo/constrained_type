use regex::Regex;

pub type CtorResult<T> = Result<Option<T>, String>;

pub fn new_string<'a, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    raw: &'a str,
) -> CtorResult<T>
    where
        F: Fn(&'a str) -> T
{
    if raw.len() == 0 {
        return Err(format!("{} must not be empty", field_name));
    }

    if raw.len() > max_len {
        return Err(format!("{} must not be greater than {} characters", field_name, max_len));
    }

    Ok(Option::from(ctor(raw)))
}

pub fn new_string_option<'a, T, F>(
    field_name: &str,
    ctor: F,
    max_len: usize,
    str: Option<&'a str>,
) -> CtorResult<T>
    where
        F: Fn(&'a str) -> T
{
    if str == None {
        return Ok(None);
    }

    if str.unwrap().len() > max_len {
        return Err(format!("{} must not be greater than {} characters", field_name.to_string(), max_len));
    }

    Ok(Some(ctor(str.unwrap())))
}

pub fn new_string_like<'a, T, F>(
    field_name: &str,
    ctor: F,
    pattern: &str,
    str: &'a str,
) -> CtorResult<T>
    where
        F: Fn(&'a str) -> T
{
    if Regex::new(pattern).unwrap().is_match(str) {
        return Ok(Some(ctor(str)));
    }

    Err(format!(
        "'{}': '{}' must match the pattern '{}'",
        field_name,
        str,
        pattern
    ))
}

pub fn new_int<T, F>(
    field_name: &str,
    ctor: F,
    min_val: isize,
    max_val: isize,
    int: isize,
) -> CtorResult<T>
    where
        F: Fn(isize) -> T
{
    if int < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if int > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(int)))
}

pub fn new_uint<T, F>(
    field_name: &str,
    ctor: F,
    min_val: usize,
    max_val: usize,
    int: usize,
) -> CtorResult<T>
    where
        F: Fn(usize) -> T
{
    if int < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if int > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(int)))
}

pub fn new_decimal<T, F>(
    field_name: &str,
    ctor: F,
    min_val: f64,
    max_val: f64,
    decimal: f64,
) -> CtorResult<T>
    where
        F: Fn(f64) -> T
{
    if decimal < min_val {
        return Err(format!("{} must not be less than {}", field_name, min_val));
    }

    if decimal > max_val {
        return Err(format!("{} must not be greater than {}", field_name, max_val));
    }

    Ok(Some(ctor(decimal)))
}
