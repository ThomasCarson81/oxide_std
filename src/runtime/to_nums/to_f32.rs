use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_i8_f32(value: i8) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_i16_f32(value: i16) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_i32_f32(value: i32) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_i64_f32(value: i64) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_u8_f32(value: u8) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_u16_f32(value: u16) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_u32_f32(value: u32) -> f32 {
    value as f32
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_u64_f32(value: u64) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_f32_f32(value: f32) -> f32 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_f64_f32(value: f64) -> f32 {
    value as f32
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f32_string_f32(value: OxideString) -> f32 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
