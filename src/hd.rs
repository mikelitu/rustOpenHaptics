pub mod hd_vars;

pub struct HapticDevice {
    pub lib: libloading::Library,
    pub position: Vec<f32>,
    pub joints: Vec<f32>,
    pub gimbals: Vec<f32>,
    pub button: bool
}

impl HapticDevice {
    pub fn init(&self) -> Result<hd_vars::HHD, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn() -> hd_vars::HHD> = self.lib.get(b"hdInitDevice")?;
            Ok(func())
        }
        
    }

    pub fn close(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn() -> u32> = self.lib.get(b"hdDisableDevice")?;
            Ok(func())
        }
    }

    pub fn position(&self) -> Vec<hd_vars::HDdouble> {
        self._get_doublev(hd_vars::HD_GET_CURRENT_TRANSFORM)
    }

    pub fn button(&self) -> hd_vars::HDint {
        self._get_integerv(hd_vars::HD_GET_CURRENT_BUTTONS)
    }

    fn _get_doublev(&self, code: hd_vars::HDenum) -> Vec<hd_vars::HDdouble> {
        unsafe {
            let data = Vec::new();
            let func: libloading::Symbol<unsafe extern fn(hd_vars::HDenum, &Vec<hd_vars::HDdouble>) -> hd_vars::HDvoid> = self.lib.get(b"hdGetDoublev").unwrap();
            func(code, &data);
            data
        }
    }

    fn _get_integerv(&self, code: hd_vars::HDenum) -> hd_vars::HDint {
        unsafe {
            let data: hd_vars::HDint = hd_vars::HDint(0);
            let func: libloading::Symbol<unsafe extern "C" fn(hd_vars::HDenum, &hd_vars::HDint) -> hd_vars::HDvoid> = self.lib.get(b"hdGetIntegerv").unwrap();
            func(code, &data);
            data
        }
    }

    pub fn _begin_frame(&self, id: hd_vars::HHD) -> Result<hd_vars::HDvoid, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(hd_vars::HHD) -> hd_vars::HDvoid> = self.lib.get(b"hdBeginFrame")?;
            Ok(func(id))
        }
    }

    pub fn _end_frame(&self, id: hd_vars::HHD) -> Result<hd_vars::HDvoid, Box<dyn std::error::Error>> {
        unsafe {
            let func: libloading::Symbol<unsafe extern fn(hd_vars::HHD) -> hd_vars::HDvoid> = self.lib.get(b"hdEndFrame")?;
            Ok(func(id))
        }
    }

}


pub fn open_library(lib_name: &str) -> Result<libloading::Library, libloading::Error> {
    unsafe {
        libloading::Library::new(lib_name)
    }
}



