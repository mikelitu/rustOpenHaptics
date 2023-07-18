mod hd;
use std::{thread, time};

fn main() {
    
    let lib_name = "libHD.so";

    let device = hd::HapticDevice{
        position: Vec::with_capacity(3),
        joints: Vec::with_capacity(3),
        gimbals: Vec::with_capacity(3),
        button: false,
        lib: hd::open_library(lib_name).unwrap()
    };
    let id = device.init().unwrap();
    println!("The ID from the connected device is {:?}", id);
    
    device._begin_frame(id);

    let mut i = 0;
    while i < 1000 {
        let button = device.button();
        println!("{:?}", button);
        i = i + 1;
        thread::sleep(time::Duration::from_millis(1))
    }

    device.close().unwrap();
}