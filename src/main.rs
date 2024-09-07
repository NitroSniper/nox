use std::{os::unix::process::CommandExt, process::Command};

use clap::{builder::styling, ArgAction, Parser, Subcommand, ValueEnum};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

/// Assuming this is gonna be hosted on github
struct FlakeGitHubURL<'a> {
    name: &'a str,
    repo: &'a str,
    branch: &'a str,
}

/// These are the dev shells option that can be passed in, Default will be Chain
enum DevShellOption {
    /// This contains certain programs that helps with developing with the language
    /// e.g. Formatters, LSP, CLI programs
    Utility,

    /// This only includes the build chain for the language
    Chain,

    /// This includes both chain and util programs
    Battery,
}

impl DevShellOption {
    fn from_flags(chain: bool, util: bool) -> Self {
        match (chain, util) {
            (true, true) => Self::Battery,
            (true, false) => Self::Chain,
            (false, true) => Self::Utility,
            // Default case
            (false, false) => Self::Battery,
        }
    }
    fn get_flake_option(&self) -> &str {
        match self {
            DevShellOption::Utility => "util",
            DevShellOption::Chain => "chain",
            DevShellOption::Battery => "battery",
        }
    }
}

impl FlakeGitHubURL<'_> {
    fn get_flake(&self, lang: Language, option: DevShellOption) -> String {
        format!(
            "github:{}/{}?dir={}{}",
            self.name,
            self.repo,
            lang.get_name(),
            format!("#{}", option.get_flake_option())
        )
    }
    fn get_flake_template(&self, lang: Language) -> String {
        format!("github:{}/{}#{}", self.name, self.repo, lang.get_name())
    }
    fn get_raw_flake(&self, lang: Language) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}/flake.nix",
            self.name,
            self.repo,
            self.branch,
            lang.get_name()
        )
    }
}

const MY_GIT: FlakeGitHubURL = FlakeGitHubURL {
    name: "NitroSniper",
    repo: "nox",
    branch: "main",
};

/// A Nix tool to quick start development and packaging applications.
///
/// This tool is intended for personal use, but anyone can use it.
/// This tool helps you quickly enter development shells, initialize projects with language-specific templates, and package applications
#[derive(Parser)]
#[command(version, about, styles = STYLES)]
struct Args {
    #[command(subcommand)]
    command: NoxCommands,
}

#[derive(Subcommand)]
enum NoxCommands {
    /// Enter a development shell of language, This command only works on Unix
    #[command(arg_required_else_help = true, short_flag = 'D')]
    Develop {
        /// Programming Language being targetted for development
        #[arg(value_enum, required = true)]
        language: Language,

        /// Include the language build chain?
        #[arg(action = ArgAction::SetTrue, short = 'c')]
        chain: bool,

        /// Include utilities that help with development?
        #[arg(action = ArgAction::SetTrue, short = 'u')]
        utilities: bool,
    },

    /// Initialise directory with template files
    #[command(arg_required_else_help = true, short_flag = 'I')]
    Init {
        #[arg(value_enum)]
        language: Language,
    },
    /// Build application
    #[command(arg_required_else_help = true, short_flag = 'B')]
    Build {
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
    fn get_name<'a>(&self) -> &str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        NoxCommands::Develop {
            language,
            chain,
            utilities,
        } => {
            Command::new("nix")
                .arg("develop")
                .arg(MY_GIT.get_flake(language, DevShellOption::from_flags(chain, utilities)))
                // No lock file since this is just a development shell
                .arg("--no-write-lock-file")
                .exec();
        }
        NoxCommands::Init { language } => {
            Command::new("nix")
                .args(["flake", "init", "--template"])
                .arg(MY_GIT.get_flake_template(language))
                .spawn()
                .expect("Command to execute")
                .wait()
                .expect("Command to complete");
        }
        NoxCommands::Build { language } => {
            let res = ureq::get(&MY_GIT.get_raw_flake(language))
                .call()
                .expect("Response to be delivered");
            let text = res.into_string().unwrap();
            println!("Raw TExt: {}", text);
        }
    }
}
