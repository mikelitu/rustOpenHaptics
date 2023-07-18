use std::{ffi::*, os::raw::c_uint};

#[derive(Debug)]
pub struct HDenum(c_uint);
#[derive(Debug)]
pub struct HDint(pub c_int);
#[derive(Debug)]
pub struct HHD(c_uint);
#[derive(Debug)]
pub struct HDvoid(c_void);
#[derive(Debug)]
pub struct HDdouble(c_double);

pub const HD_GET_CURRENT_BUTTONS: HDenum = HDenum(0x2000);
pub const HD_GET_CURRENT_TRANSFORM: HDenum = HDenum(0);

