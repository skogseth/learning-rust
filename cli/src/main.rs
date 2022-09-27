use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

use anyhow::Error as AnyError;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(value_parser)]
    max_value: usize,

    #[clap(short, long, value_parser, default_value = "data.txt")]
    filename: String,

    /// Set verbosity
    #[clap(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,
}


fn main() -> Result<(), AnyError> {
    let args = Args::parse();

    let log_level = match args.verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    Builder::new().filter_level(log_level).init();

    log::info!("hello world");
    log::warn!("this is dangerous!!");
    
    let vals = some_squared(args.max_value);

    let mut str = String::new();
    for val in vals {
        log::debug!("val: {}", val);
        str += &val.to_string();
        str.push('\t');
    }

    if args.filename != "data.txt" {
        log::debug!("filename: {}", args.filename);
        log::error!("FUUUUUUCK");
        anyhow::bail!("abandon ship!")
    }

    Ok(())
}

fn some_squared(max_value: usize) -> Vec<i32> {
    (0..max_value)
        .filter_map(|x| {
            let x = x as i32;
            let squared = x * x;
            if squared % 2 == 0 {
                Some(squared)
            } else {
                log::info!("value {} was filtered out", squared);
                None
            }
        })
        .collect::<Vec<_>>()
}
