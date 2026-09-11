#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_i8_i8_i8_i8(value: i8, min: i8, max: i8) -> i8 {
    i8::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_i16_i16_i16_i16(value: i16, min: i16, max: i16) -> i16 {
    i16::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_i32_i32_i32_i32(value: i32, min: i32, max: i32) -> i32 {
    i32::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_i64_i64_i64_i64(value: i64, min: i64, max: i64) -> i64 {
    i64::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_u8_u8_u8_u8(value: u8, min: u8, max: u8) -> u8 {
    u8::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_u16_u16_u16_u16(value: u16, min: u16, max: u16) -> u16 {
    u16::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_u32_u32_u32_u32(value: u32, min: u32, max: u32) -> u32 {
    u32::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_u64_u64_u64_u64(value: u64, min: u64, max: u64) -> u64 {
    u64::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_f32_f32_f32_f32(value: f32, min: f32, max: f32) -> f32 {
    f32::clamp(value, min, max)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_clamp_f64_f64_f64_f64(value: f64, min: f64, max: f64) -> f64 {
    f64::clamp(value, min, max)
}
