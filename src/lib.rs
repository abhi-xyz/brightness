use ddc_hi::{Ddc, Display};
use log::*;

pub fn get_device() {
    let displays = Display::enumerate();

    for mut display in displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(_result) => println!("Connected device {}", display.info),
            Err(_) => warn!("Connection to device {} Failed", display.info),
        }
    }
}
