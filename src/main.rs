use brightness::get_device;
use clap::{Parser, Subcommand};
use ddc_hi::{Ddc, Display};
use log::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    GetDevice,
    #[command(short_flag = 'g')]
    GetBrightness,
    #[command(short_flag = 's')]
    SetBrightness {
        value: i16,
    },
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Command::GetDevice => {
            println!("Device info:");
            get_device();
        }
        Command::GetBrightness => {
            println!("Current brightness:");
            get_brightness();
        }
        Command::SetBrightness { value } => {
            println!("Setting Brightness:");
            let new_value = value;
            trace!("This value is got from clap: {}", &new_value);
            set_brightness(new_value);
        }
    }
}

fn calc(value: i16) -> i16 {
    if value >= 100 {
        // if the value doesn't fit in 0 - 100 is returned.
        println!("value must be in between 0 - 100");
        let value = 100;
        println!("value setted to {}", &value);
        return value;
    } else if value <= 0 {
        // if the value doesn't fit in 0 - 100 is returned.
        println!("value must be in between 0 - 100");
        let value = 0;
        println!("value setted to {}", &value);
        return value;
    } else if value > 0 || value < 100 {
        // if the value doesn't fit in 0 - 100 is returned.
        println!("value is: {}", &value);
        return value;
    }
    0
}

fn set_brightness(value: i16) {
    let calc_value = calc(value);
    let mut displays = Display::enumerate();
    for display in &mut displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(_current_value) => match display.handle.set_vcp_feature(
                0x10,
                calc_value.try_into().expect("Failed to set brightness"),
            ) {
                Ok(_) => println!(
                    "Brightness adjusted to {} on display {:?}",
                    calc_value, display.info.model_name
                ),
                Err(err) => eprintln!(
                    "Failed to set brightness on display {:?}\nerror: {:?}",
                    display.info.model_name, err
                ),
            },
            Err(_) => eprintln!(
                "Failed to get current brightness for display {:?}",
                display.info.model_name
            ),
        }
    }
}

fn get_brightness() -> i16 {
    let displays = Display::enumerate();

    for mut display in displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(result) => {
                println!("Current Brightness is {}", result.value());
                return result.value() as i16;
            }
            Err(_) => println!("Err from get_brightness function"),
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};

    use crate::{get_brightness, set_brightness};

    #[test]
    fn set_brightness_test() {
        let test_value: i16 = 10;
        set_brightness(test_value);
        sleep(Duration::from_secs(5));
        assert_eq!(get_brightness(), 10);
        sleep(Duration::from_secs(5));
    }
    #[test]
    fn set_brightness_test_higher() {
        let test_value: i16 = 130;
        set_brightness(test_value);
        sleep(Duration::from_secs(5));
        assert_ne!(get_brightness(), 130);
        sleep(Duration::from_secs(5));
    }
    #[test]
    fn set_brightness_test_higher_eq() {
        let test_value: i16 = 130;
        set_brightness(test_value);
        sleep(Duration::from_secs(5));
        assert_eq!(get_brightness(), 100);
        sleep(Duration::from_secs(5));
    }
    #[test]
    fn set_brightness_test_lower() {
        let test_value: i16 = -2;
        set_brightness(test_value);
        sleep(Duration::from_secs(5));
        assert_ne!(get_brightness(), -2);
        sleep(Duration::from_secs(5));
    }
    #[test]
    fn set_brightness_test_lower_eq() {
        let test_value: i16 = -2;
        set_brightness(test_value);
        sleep(Duration::from_secs(5));
        assert_eq!(get_brightness(), 0);
        sleep(Duration::from_secs(5));
    }
}
