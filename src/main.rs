use chrono::Timelike;
use clap::Parser;

/// Execute a program at the given time
#[derive(Parser)]
struct Cli {
    /// Specify the hour
    #[arg(long, value_parser = clap::value_parser!(u32).range(0..23))]
    hour: u32,

    /// Specify the minute
    #[arg(long, value_parser = clap::value_parser!(u32).range(0..59))]
    minute: u32,

    /// Specify the program
    program: String,

    /// Specify arguments of the program
    args: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let duration = duration_until(cli.hour, cli.minute);

    std::thread::sleep(duration);

    std::process::Command::new(cli.program)
        .args(cli.args)
        .spawn()?;

    Ok(())
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
