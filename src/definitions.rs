#[repr(C)]
pub struct OxideString {
    pub data: *const u8,
    pub length: usize,
}
impl OxideString {
    pub fn to_string(&self) -> String {
        unsafe {
            let bytes = std::slice::from_raw_parts(self.data as *const u8, self.length);

            String::from_utf8(bytes.to_vec()).unwrap()
        }
    }
    pub fn from_string(string: String) -> OxideString {
        let bytes = string.into_bytes();

        let length = bytes.len();
        let data = bytes.as_ptr();

        std::mem::forget(bytes);

        OxideString { data, length }
    }
}
