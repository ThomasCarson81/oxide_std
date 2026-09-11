use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_i8_u64(value: i8) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_i16_u64(value: i16) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_i32_u64(value: i32) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_i64_u64(value: i64) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_u8_u64(value: u8) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_u16_u64(value: u16) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_u32_u64(value: u32) -> u64 {
    value as u64
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_u64_u64(value: u64) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_f32_u64(value: f32) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_f64_u64(value: f64) -> u64 {
    value as u64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u64_string_u64(value: OxideString) -> u64 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
