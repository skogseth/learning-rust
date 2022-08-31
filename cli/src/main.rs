use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Name of the person to greet
    #[clap(short, long, value_parser)]
    name: String,

    // Length of pause before greet
    #[clap(short, long, value_parser, default_value_t = 3)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    let mut str = String::new();
    for _ in 0..args.count {
        str.push('.');
    }

    println!("Hello{} {}", str, args.name);
}