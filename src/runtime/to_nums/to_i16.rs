use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_i8_i16(value: i8) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_i16_i16(value: i16) -> i16 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_i32_i16(value: i32) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_i64_i16(value: i64) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_u8_i16(value: u8) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_u16_i16(value: u16) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_u32_i16(value: u32) -> i16 {
    value as i16
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_u64_i16(value: u64) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_f32_i16(value: f32) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_f64_i16(value: f64) -> i16 {
    value as i16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i16_string_i16(value: OxideString) -> i16 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
