use brightness::{get_brightness, get_device, set_brightness};
use clap::{Parser, Subcommand};
use log::*;

#[derive(Parser, Debug)]
#[command(version, about = "A simple brightness app which uses ddc",author = "Abhinandh S", long_about = None)]
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
            get_device();
        }
        Command::GetBrightness => {
            get_brightness();
        }
        Command::SetBrightness { value } => {
            let new_value = value;
            trace!("This value is got from clap: {}", &new_value);
            set_brightness(new_value);
        }
    }
}

#[cfg(test)]
mod tests {

    use brightness::calc;

    #[test]
    fn test_calc() {
        assert_eq!(calc(110), 100);
        assert_eq!(calc(-10), 0);
        assert_eq!(calc(50), 50);
        assert_eq!(calc(0), 0);
        assert_eq!(calc(100), 100);
    }
}
