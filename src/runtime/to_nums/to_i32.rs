use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_i8_i32(value: i8) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_i16_i32(value: i16) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_i32_i32(value: i32) -> i32 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_i64_i32(value: i64) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_u8_i32(value: u8) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_u16_i32(value: u16) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_u32_i32(value: u32) -> i32 {
    value as i32
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_u64_i32(value: u64) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_f32_i32(value: f32) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_f64_i32(value: f64) -> i32 {
    value as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_i32_string_i32(value: OxideString) -> i32 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
