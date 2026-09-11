#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_i8_i8_i8(a: i8, b: i8) -> i8 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_i16_i16_i16(a: i16, b: i16) -> i16 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_i32_i32_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_i64_i64_i64(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_u8_u8_u8(a: u8, b: u8) -> u8 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_u16_u16_u16(a: u16, b: u16) -> u16 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_u32_u32_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_u64_u64_u64(a: u64, b: u64) -> u64 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_f32_f32_f32(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_min_f64_f64_f64(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
