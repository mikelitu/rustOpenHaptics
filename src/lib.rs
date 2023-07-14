pub fn open_library(lib_name: &str) -> Result<libloading::Library, libloading::Error> {
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
