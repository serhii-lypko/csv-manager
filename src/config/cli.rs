use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(long, default_value_t = false)]
    with_header: bool,
}

pub struct Config {
    pub file_path: String,
    pub with_header: bool,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            file_path: args.path,
            with_header: args.with_header,
        }
    }
}

pub fn parse_args() -> Config {
    let args = Args::parse();
    args.into()
}

// cargo run -- --path ~/Projects/csv-manager/data-samples/books.csv --with-header
