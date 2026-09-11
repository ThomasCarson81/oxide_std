use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_i8_u8(value: i8) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_i16_u8(value: i16) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_i32_u8(value: i32) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_i64_u8(value: i64) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_u8_u8(value: u8) -> u8 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_u16_u8(value: u16) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_u32_u8(value: u32) -> u8 {
    value as u8
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_u64_u8(value: u64) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_f32_u8(value: f32) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_f64_u8(value: f64) -> u8 {
    value as u8
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u8_string_u8(value: OxideString) -> u8 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
