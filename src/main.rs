use rust_open_haptics::init_device;

fn main() {
    println!("The ID from the connected device is {:?}", init_device().unwrap());
}