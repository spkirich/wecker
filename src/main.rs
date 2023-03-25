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
    println!("{}:{}", args.hour, args.minute);
}
