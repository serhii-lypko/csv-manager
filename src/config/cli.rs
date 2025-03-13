use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    // TODO:
    // with_header: bool
}

pub struct Config {
    pub file_path: String,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            file_path: args.path,
        }
    }
}

pub fn parse_args() -> Config {
    let args = Args::parse();
    args.into()
}
