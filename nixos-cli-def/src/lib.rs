pub mod commands;

use clap::{ArgAction, Parser, Subcommand};
use commands::{completions::CompletionsArgs, rebuild::RebuildArgs};

#[derive(Parser, Debug)]
#[command(
	name = "nilla-nixos",
	version,
	long_about = None,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[arg(
		long,
		short,
		help = "The nilla project to use (check Valid project sources in the man pages)",
		value_hint = clap::ValueHint::AnyPath,
		default_value = "./",
		global = true
	)]
    pub project: String,
    #[arg(
        long,
        short,
		action = ArgAction::Count,
        help = "The verbosity level to use",
        global = true
    )]
    pub verbose: u8,
    #[arg(
        long,
        short,
		action = ArgAction::SetTrue,
        help = "Quiet level of the program",
        global = true
    )]
    pub quiet: bool,
    #[arg(
        long,
		action = ArgAction::SetTrue,
        help = "Log any ran eval commands",
        global = true,
		default_value_t = false,
    )]
    pub show_eval_commands: bool,
}

#[derive(Subcommand, Debug)]
#[command(allow_external_subcommands = true)]
pub enum Commands {
    #[command(flatten)]
    Rebuild(RebuildCommands),
    #[command(alias = "completion")]
    Completions(CompletionsArgs),
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[derive(Subcommand, Debug)]
pub enum RebuildCommands {
    #[command(about = "Build, install, and switch into a system")]
    Switch(RebuildArgs),
    #[command(about = "Build and install a system, making it the default boot target")]
    Boot(RebuildArgs),
    #[command(about = "Test a system")]
    Test(RebuildArgs),
    #[command(about = "Build a system")]
    Build(RebuildArgs),
    #[command(about = "Show what would be built or downloaded when building the system")]
    DryBuild(RebuildArgs),
    #[command(about = "Build a system, and show what changes would be made by activating it")]
    DryActivate(RebuildArgs),
    #[command(about = "Open a system configuration in a repl")]
    Repl(RebuildArgs),
    #[command(about = "Build a disk image from a system")]
    BuildImage(RebuildArgs),
    #[command(about = "Build a script that starts a NixOS VM with a system's configuration")]
    BuildVm(RebuildArgs),
    #[command(about = "Like build-vm, but uses the bootloader of the configuration")]
    BuildVmWithBootloader(RebuildArgs),
    #[command(about = "List the available generations, similar to the bootloader")]
    ListGenerations(RebuildArgs),
}

// I hate this, but it's better than doing some weird stuff
impl RebuildCommands {
    pub fn get_args(&self) -> Option<&RebuildArgs> {
        match &self {
            Self::Switch(args) => Some(args),
            Self::Boot(args) => Some(args),
            Self::Test(args) => Some(args),
            Self::Build(args) => Some(args),
            Self::DryBuild(args) => Some(args),
            Self::DryActivate(args) => Some(args),
            Self::Repl(args) => Some(args),
            Self::BuildImage(args) => Some(args),
            Self::BuildVm(args) => Some(args),
            Self::BuildVmWithBootloader(args) => Some(args),
            Self::ListGenerations(args) => Some(args),
        }
    }
}
