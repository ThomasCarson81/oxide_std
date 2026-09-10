use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_u8_string(value: u8) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_u16_string(value: u16) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_u32_string(value: u32) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_u64_string(value: u64) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_i8_string(value: i8) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_i16_string(value: i16) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_i32_string(value: i32) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_i64_string(value: i64) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_f32_string(value: f32) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_string_f64_string(value: f64) -> OxideString {
    let value_string = value.to_string();
    OxideString::from_string(value_string)
}
