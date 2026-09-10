#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_i8_i8(value: i8) -> i8 {
    value.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_i16_i16(value: i16) -> i16 {
    value.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_i32_i32(value: i32) -> i32 {
    value.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_i64_i64(value: i64) -> i64 {
    value.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_f32_f32(value: f32) -> f32 {
    value.abs()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_abs_f64_f64(value: f64) -> f64 {
    value.abs()
}
