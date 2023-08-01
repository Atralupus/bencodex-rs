pub mod codec;

use std::os::raw::c_char;
use std::ffi::{CStr, CString};

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
