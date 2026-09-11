use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_string_concat(a: OxideString, b: OxideString) -> OxideString {
    let mut result = a.to_string().as_str().to_owned();
    result.push_str(b.to_string().as_str());
    OxideString::from_string(result)
}
