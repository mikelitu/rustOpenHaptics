use std::{ffi::*, os::raw::c_uint};

pub struct HapticDevice {
    pub lib: libloading::Library,
    pub position: Vec<f32>,
    pub joints: Vec<f32>,
    pub gimbals: Vec<f32>,
    pub button: bool
}

impl HapticDevice {
    pub fn init(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn() -> u32> = self.lib.get(b"hdInitDevice")?;
            Ok(func())
        }
        
    }

    pub fn close(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn() -> u32> = self.lib.get(b"hdDisableDevice")?;
            Ok(func())
        }
    }

    pub fn position(&self) -> Vec<f64> {
        self._get_doublev(200)
    }

    pub fn button(&self) -> i32 {
        self._get_integerv(0x2000)
    }

    fn _get_doublev(&self, code: u32) -> Vec<c_double> {
        unsafe {
            let data = Vec::new();
            let func: libloading::Symbol<unsafe extern fn(c_uint, &Vec<c_double>) -> c_void> = self.lib.get(b"hdGetDoublev").unwrap();
            func(code, &data);
            data
        }
    }

    fn _get_integerv(&self, code: c_uint) -> c_int {
        unsafe {
            let data: c_int = 0;
            let func: libloading::Symbol<unsafe extern "C" fn(c_uint, &c_int) -> c_void> = self.lib.get(b"hdGetIntegerv").unwrap();
            func(code, &data);
            println!("{}", data);
            data
        }
    }

    pub fn _begin_frame(&self, id: c_uint) -> Result<c_void, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(c_uint) -> c_void> = self.lib.get(b"hdBeginFrame")?;
            Ok(func(id))
        }
    }

    pub fn _end_frame(&self, id: c_uint) -> Result<c_void, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(c_uint) -> c_void> = self.lib.get(b"hdEndFrame")?;
            Ok(func(id))
        }
    }

}


pub fn open_library(lib_name: &str) -> Result<libloading::Library, libloading::Error> {
    unsafe {
        libloading::Library::new(lib_name)
    }
}



