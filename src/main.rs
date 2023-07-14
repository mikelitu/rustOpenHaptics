use rust_open_haptics::HapticDevice;

fn main() {
    
    let device = HapticDevice{
        lib_name: String::from("libHD.so"),
        position: Vec::with_capacity(3),
        joints: Vec::with_capacity(3),
        gimbals: Vec::with_capacity(3),
        button: false
    };
    println!("The ID from the connected device is {:?}", device.init().unwrap());
    device.close().unwrap();
}