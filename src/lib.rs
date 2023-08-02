pub mod codec;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub use codec::decode::{Decode, DecodeError};
pub use codec::encode::Encode;
pub use codec::types::{
    BencodexDictionary, BencodexKey, BencodexList, BencodexValue, BENCODEX_NULL,
};

#[no_mangle]
pub extern "C" fn c_encode(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let rust_str = c_str.to_str().unwrap();
    let rust_string = rust_str.to_string();

    let mut output: Vec<u8> = Vec::new();
    rust_string.encode(&mut output).unwrap();

    let c_string = CString::new(&output[..]).unwrap();
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn c_free(str: *mut c_char) {
    unsafe { CString::from_raw(str) };
}

#[no_mangle]
pub extern "C" fn c_decode(input_bytes: *const u8, length: usize) -> *mut c_char {
    let data: &[u8] = unsafe { std::slice::from_raw_parts(input_bytes, length) };
    let result_string: String = match data.decode() {
        Ok(value) => format!("{:?}", value),
        Err(_) => String::from("Failure"),
    };

    let c_string = CString::new(result_string).unwrap();
    c_string.into_raw()
}
