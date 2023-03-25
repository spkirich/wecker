use chrono::Timelike;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, value_parser = clap::value_parser!(u32).range(0..23))]
    hour: u32,

    #[arg(long, value_parser = clap::value_parser!(u32).range(0..59))]
    minute: u32,
}

fn main() {
    let cli = Cli::parse();
    let duration = duration_until(cli.hour, cli.minute);

    std::thread::sleep(duration);
    println!("Now is the time!");
}

fn duration_until(hour: u32, minute: u32) -> std::time::Duration {
    let now = chrono::Local::now();

    let mut then = now
        .with_hour(hour)
        .unwrap()
        .with_minute(minute)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();

    if now > then {
        then += chrono::Duration::days(1);
    }

    (then - now).to_std().unwrap()
}
