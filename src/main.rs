use std::{os::unix::process::CommandExt, process::Command};

use clap::{Parser, Subcommand, ValueEnum};

/// A Nix tool to quickstart development and packaging applications
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: NoxCommands,
}

#[derive(Subcommand)]
enum NoxCommands {
    /// Enter a development shell of chosen language
    #[command(arg_required_else_help = true)]
    Develop {
        /// Language to be chosen
        #[arg(value_enum)]
        language: Language,
    },

    /// Initialise Current Directory with template
    #[command(arg_required_else_help = true)]
    Init {
        /// Language to be chosen
        #[arg(value_enum)]
        language: Language,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Language {
    Rust,
    Python,
    // Nix,
    // Go,
    // Lua,
    // C,
    // #[clap(name = "c++")]
    // CPlusPlus,
}

impl Language {
    fn get_name<'a>(&'a self) -> &'a str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
        }
    }
    fn get_flake(&self) -> String {
        format!("github:NitroSniper/nox?dir={}", self.get_name())
    }
    fn get_flake_template(&self) -> String {
        format!("github:NitroSniper/nox#{}", self.get_name())
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        NoxCommands::Develop { language } => {
            Command::new("nix")
                .arg("develop")
                .arg(language.get_flake())
                // No lock file since this is just a development shell
                .arg("--no-write-lock-file")
                .exec();
        }
        NoxCommands::Init { language } => {
            Command::new("nix")
                .args(["flake", "init", "--template"])
                .arg(language.get_flake_template())
                .spawn()
                .expect("Command to execute")
                .wait()
                .expect("Command to complete");
        }
    }
}
