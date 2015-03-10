use libc::c_char;
use std::ffi;
use std::str;
use std::vec;
use std::mem;
use std::boxed::Box;

extern crate libc;

struct TokenCollection {
	parts: Vec<&str>
}

impl TokenCollection {
	fn new(str: &str) -> &TokenCollection {
		TokenCollection {
			parts: str.split(|c| c.is_whitespace()).collect()
		}
	}

	fn len(&self) -> usize {
		self.parts.len()
	}
}

// impl Drop for TokenCollection {
// 	fn drop(&self) {
// 		println!("drop it like it's hot");
// 	}
// }

fn c_str_to_slice(c_str: *mut c_char) -> &str {
	let c_bytes = unsafe { ffi::c_str_to_bytes(&c_str); };
	str::from_utf8(c_bytes).unwrap()
}

#[no_mangle]
pub extern fn token_collection_create(c_str: *mut c_char) -> *mut TokenCollection {
	let str = c_str_to_slice(c_str);

	unsafe { mem::transmute(Box::new(TokenCollection::new(str))) }
}

#[no_mangle]
pub extern fn token_collection_len(pimpl: *mut TokenCollection) -> libc::uint64_t {
	pimpl.len()
}

#[no_mangle]
pub extern fn token_collection_destroy(pimpl: *mut TokenCollection) {

	let _drop_me = unsafe { Box::from_raw(pimpl) };
}
