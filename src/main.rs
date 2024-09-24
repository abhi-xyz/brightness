use brightness::{change_brightness, get_brightness, get_device, set_brightness};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about = "A simple brightness app which uses ddc",author = "Abhinandh S", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(short_flag = 'd', long_flag = "detect")]
    GetDevice,
    #[command(short_flag = 'g', long_flag = "get")]
    GetBrightness,
    #[command(short_flag = 's', long_flag = "set")]
    SetBrightness { value: i16 },
    #[command(short_flag = 'c', long_flag = "change")]
    ChangeBrightness { sign: String, value: i16 },
}

fn main() {
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
            dbg!("This value is got from clap: {}", &new_value);
            set_brightness(new_value);
        }
        Command::ChangeBrightness { value, sign } => {
            let new_value = value;
            let change_sign = sign;
            dbg!("This value is got from clap: {}", &new_value);
            dbg!("This value is got from clap: {}", &change_sign);
            change_brightness(new_value, change_sign);
        }
    }
}

#[cfg(test)]
mod tests {
    use brightness::calc;

    #[test]
    fn test_calc() {
        dbg!("This value is got from clap:");
        assert_eq!(calc(110), 100);
        assert_eq!(calc(-10), 0);
        assert_eq!(calc(50), 50);
        assert_eq!(calc(0), 0);
        assert_eq!(calc(100), 100);
    }
}
