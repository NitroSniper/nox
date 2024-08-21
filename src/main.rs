use clap::{Parser, Subcommand, ValueEnum};

/// A Nix tool to quickstart development and packaging applications
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter a development shell of chosen language
    Enter {
        /// Language to be chosen
        #[arg(value_enum)]
        lang: Language,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Language {
    Rust,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Enter { lang } => {
            println!("You suck at {:?}", lang);
        }
    }
}
