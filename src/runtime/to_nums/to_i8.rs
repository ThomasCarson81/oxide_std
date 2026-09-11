use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_i8_i8(value: i8) -> i8 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_i16_i8(value: i16) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_i32_i8(value: i32) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_i64_i8(value: i64) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_u8_i8(value: u8) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_u16_i8(value: u16) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_u32_i8(value: u32) -> i8 {
    value as i8
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_u64_i8(value: u64) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_f32_i8(value: f32) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_f64_i8(value: f64) -> i8 {
    value as i8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i8_string_i8(value: OxideString) -> i8 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
