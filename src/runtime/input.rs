use crate::definitions::OxideString;
use prompted::input;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_input_string() -> OxideString {
    let input = input!();

    OxideString::from_string(input)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_input_string_string(message: OxideString) -> OxideString {
    let input = input!("{}", message.to_string());

    OxideString::from_string(input)
}
