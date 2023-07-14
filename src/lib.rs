pub struct HapticDevice {
    pub lib_name: String,
    pub position: Vec<f32>,
    pub joints: Vec<f32>,
    pub gimbals: Vec<f32>,
    pub button: bool
}

impl HapticDevice {
    pub fn init(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let lib = open_library(&self.lib_name)?;
            let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"hdInitDevice")?;
            Ok(func())
        }
        
    }

    pub fn close(&self) -> Result<u32, Box<dyn std::error::Error>> {
        unsafe {
            let lib = open_library(&self.lib_name)?;
            let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"hdDisableDevice")?;
            Ok(func())
        }
    }
}


fn open_library(lib_name: &str) -> Result<libloading::Library, libloading::Error> {
    unsafe {
        libloading::Library::new(lib_name)
    }
}

pub fn init_device() -> Result<u32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = open_library("libHD.so")?;
        let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"hdInitDevice")?;
        Ok(func())
    }
}
