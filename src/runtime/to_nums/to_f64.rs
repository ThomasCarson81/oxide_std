use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_i8_f64(value: i8) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_i16_f64(value: i16) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_i32_f64(value: i32) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_i64_f64(value: i64) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_u8_f64(value: u8) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_u16_f64(value: u16) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_u32_f64(value: u32) -> f64 {
    value as f64
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_u64_f64(value: u64) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_f32_f64(value: f32) -> f64 {
    value as f64
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_f64_f64(value: f64) -> f64 {
    value
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_to_f64_string_f64(value: OxideString) -> f64 {
    let string = value.to_string();
    string.parse().expect("failed to parse number")
}
