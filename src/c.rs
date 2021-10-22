#![allow(non_camel_case_types)]

use crate::parse::{parse, BiStrategy};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

#[derive(ToPrimitive)]
#[repr(C)]
pub enum BsizeBiStrategy {
    kCheckBi = 1,
    kAlwaysTen = 2,
    kAlwaysBi = 3,
    kRevertBi = 4,
}

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
pub extern "C" fn BsizeParse(p: *const c_char, bi_strategy: BsizeBiStrategy) -> BsizeRes {
    let s = unsafe { CStr::from_ptr(p) }.to_str().unwrap();
    let strategy = BiStrategy::from_i32(bi_strategy.to_i32().unwrap()).unwrap();
    match parse(s, strategy) {
        Ok(d) => BsizeRes {
            error: 0,
            num: d.0,
            unit: BsizeUnit::from_i32(d.1.to_i32().unwrap()).unwrap(),
        },
        Err(e) => BsizeRes {
            error: e.to_i32().unwrap() as c_int,
            num: 0,
            unit: BsizeUnit::kNone,
        },
    }
}
