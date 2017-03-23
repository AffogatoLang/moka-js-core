use std::ptr;
use std::ops::Add;
use std::ffi::CStr;
use std::ffi::OsStr;

use libc::c_char;
use libc::c_ushort;

use byteorder::{ LittleEndian, WriteBytesExt };

pub fn static_to_char_ptr<'a>(string: &'a str) -> *const c_char {
    let slice: &[u8] = string.as_bytes();
    let val = slice.as_ptr() as *const c_char;
    val
}

pub fn char_ptr_to_str(string: *const c_char) -> Option<&'static str> {
    let cstr: &CStr;

    unsafe {
        cstr = CStr::from_ptr(string);
    }

    match cstr.to_str() {
        Ok(valid) => Some(valid),
        Err(_) => None
    }
}

pub fn null_term_ptr_to_string(pointer: *const c_ushort) -> String {
    let mut message = String::new();

    unsafe {
        let mut offset = 0;
        loop {
            let char = ptr::read(pointer.offset(offset));
            if char == 0 {
                break;
            }
            offset += 1;

            let mut wtr: Vec<u8> = vec![];
            wtr.write_u16::<LittleEndian>(char).unwrap();
            message = message.add(CStr::from_bytes_with_nul_unchecked(wtr.as_ref()).to_str().unwrap());
        }
    }

    message
}

pub fn fixed_ptr_to_string(pointer: *const c_ushort, length: usize) -> String {
    let mut message = String::new();

    unsafe {
        let mut offset: isize = 0;
        loop {
            if (offset as usize) == length {
                break;
            }

            let char = ptr::read(pointer.offset(offset));
            offset += 1;

            let mut wtr: Vec<u8> = vec![];
            wtr.write_u16::<LittleEndian>(char).unwrap();
            message = message.add(CStr::from_bytes_with_nul_unchecked(wtr.as_ref()).to_str().unwrap());
        }
    }

    message
}

pub fn char_ptr_to_str_or(string: *const c_char, other: &'static str) -> &'static str {
    match char_ptr_to_str(string) {
        Some(str) => str,
        None => other
    }
}

pub fn osstr_to_string (input: &OsStr) -> Option<String> {
    match input.to_str() {
        None => None,
        Some(val) => Some(val.into())
    }
}