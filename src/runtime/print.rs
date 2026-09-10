use std::io::{self, Write};

use crate::definitions::OxideString;

#[unsafe(no_mangle)]
pub extern "C" fn oxide_print_string_unit(value: OxideString) {
    print!("{}", value.to_string());
    io::stdout().flush().unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_println_string_unit(value: OxideString) {
    println!("{}", value.to_string());
}
