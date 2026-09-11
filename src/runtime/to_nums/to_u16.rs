use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_i8_u16(value: i8) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_i16_u16(value: i16) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_i32_u16(value: i32) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_i64_u16(value: i64) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_u8_u16(value: u8) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_u16_u16(value: u16) -> u16 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_u32_u16(value: u32) -> u16 {
    value as u16
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_u64_u16(value: u64) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_f32_u16(value: f32) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_f64_u16(value: f64) -> u16 {
    value as u16
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u16_string_u16(value: OxideString) -> u16 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
