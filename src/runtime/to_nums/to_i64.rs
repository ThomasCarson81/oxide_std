use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_i8_i64(value: i8) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_i16_i64(value: i16) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_i32_i64(value: i32) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_i64_i64(value: i64) -> i64 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_u8_i64(value: u8) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_u16_i64(value: u16) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_u32_i64(value: u32) -> i64 {
    value as i64
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_u64_i64(value: u64) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_f32_i64(value: f32) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_f64_i64(value: f64) -> i64 {
    value as i64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i64_string_i64(value: OxideString) -> i64 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
