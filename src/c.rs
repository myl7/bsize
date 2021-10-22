#![allow(non_camel_case_types)]

use crate::parse::parse;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[derive(FromPrimitive)]
#[repr(C)]
pub enum BsizeUnit {
    kBit = 1,
    kByte = 2,
    kNone = 3,
}

#[repr(C)]
pub struct BsizeRes {
    error: c_int,
    num: u64,
    unit: BsizeUnit,
}

#[no_mangle]
pub extern "C" fn BsizeParse(p: *const c_char, ignore_bi: bool) -> BsizeRes {
    let s = unsafe { CStr::from_ptr(p) }.to_str().unwrap();
    match parse(s, ignore_bi) {
        Ok(d) => BsizeRes {
            error: 0,
            num: d.0,
            unit: FromPrimitive::from_isize(d.1 as isize).unwrap(),
        },
        Err(e) => BsizeRes {
            error: e as isize as c_int,
            num: 0,
            unit: BsizeUnit::kNone,
        },
    }
}
