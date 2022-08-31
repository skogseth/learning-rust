use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(value_parser)]
    number_of_values: usize,

    #[clap(short, long, value_parser, default_value = "data.txt")]
    filename: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let vals = (0..args.number_of_values)
        .filter_map(|x| {
            let squared = x * x;
            if squared % 2 == 0 {
                Some(squared)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut str = String::new();
    for val in vals {
        str += &val.to_string();
        str.push('\t');
    }

    let mut file = File::create(args.filename).await?;
    file.write_all(str.as_bytes()).await?;

    Ok(())
}
