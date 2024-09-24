use ddc_hi::{Ddc, Display};

pub fn calc(value: i16) -> i16 {
    if value >= 100 {
        // if the value doesn't fit in 0 - 100 is returned.
        dbg!("value must be in between 0 - 100");
        let value = 100;
        dbg!("value setted to {}", &value);
        return value;
    } else if value <= 0 {
        // if the value doesn't fit in 0 - 100 is returned.
        dbg!("value must be in between 0 - 100");
        let value = 0;
        dbg!("value setted to {}", &value);
        return value;
    } else if value > 0 || value < 100 {
        // if the value doesn't fit in 0 - 100 is returned.
        dbg!("value is: {}", &value);
        return value;
    }
    0
}

pub fn get_device() {
    let displays = Display::enumerate();

    for mut display in displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(_result) => println!("Connected device {}", display.info),
            Err(_) => println!("Connection to device {} Failed", display.info),
        }
    }
}

pub fn set_brightness(value: i16) {
    let calc_value = calc(value);
    let mut displays = Display::enumerate();
    for display in &mut displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(_current_value) => match display.handle.set_vcp_feature(
                0x10,
                calc_value.try_into().expect("Failed to set brightness"),
            ) {
                Ok(_) => println!("{}", calc_value),
                // Ok(_) => println!(
                //     "Brightness adjusted to {} on display {:?}",
                //     calc_value, display.info.model_name
                // ),
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

pub fn get_brightness() -> i16 {
    let displays = Display::enumerate();

    for mut display in displays {
        match display.handle.get_vcp_feature(0x10) {
            Ok(result) => {
                println!("{}", result.value());
                return result.value() as i16;
            }
            Err(_) => println!("Err from get_brightness function"),
        }
    }
    0
}

pub fn change_brightness(value: i16, by: String) {
    let c = get_brightness();
    let by_sing = Option::Some(by);
    match by_sing {
        Some(_) if by_sing == Some(String::from("+")) => {
            let v = c + value;
            set_brightness(value);
            println!("some + = {v}");
        }
        Some(_) if by_sing == Some(String::from("-")) => {
            let v = c - value;
            println!("some = {v}");
            set_brightness(v);
        }

        None => panic!(),
        // all other numbers
        _ => panic!(),
    }
}
