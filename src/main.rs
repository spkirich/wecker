use chrono::Timelike;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    hour: u32,

    #[arg(long)]
    minute: u32,
}

fn main() {
    let args = Args::parse();

    match duration_until(args.hour, args.minute) {
        None => {
            eprintln!("The time is wrong!");
            std::process::exit(1);
        },

        Some(duration) => {
            println!("The time is right!");
            std::thread::sleep(duration);
            println!("Now is the time!");
        },
    }
}

fn duration_until(hour: u32, minute: u32) -> Option<std::time::Duration> {
    let now = chrono::Local::now();

    let mut then = now
        .with_hour(hour)?
        .with_minute(minute)?
        .with_second(0)?
        .with_nanosecond(0)?;

    if now > then {
        then += chrono::Duration::days(1);
    }

    let duration = (then - now)
        .to_std()
        .unwrap();

    Some(duration)
}
