#[unsafe(no_mangle)]
pub extern "C" fn oxide_sqrt_f32_f32(x: f32) -> f32 {
    x.sqrt()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_sqrt_f64_f64(x: f64) -> f64 {
    x.sqrt()
}
