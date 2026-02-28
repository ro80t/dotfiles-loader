use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(name = "dfl", author = "ro80t")]
#[command(version, about, flatten_help = true)]
struct Args {}

pub fn cli() {
    let _args = Args::parse();
}
