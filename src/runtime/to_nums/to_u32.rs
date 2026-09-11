use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_i8_u32(value: i8) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_i16_u32(value: i16) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_i32_u32(value: i32) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_i64_u32(value: i64) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_u8_u32(value: u8) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_u16_u32(value: u16) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_u32_u32(value: u32) -> u32 {
    value
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_u64_u32(value: u64) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_f32_u32(value: f32) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_f64_u32(value: f64) -> u32 {
    value as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_u32_string_u32(value: OxideString) -> u32 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
