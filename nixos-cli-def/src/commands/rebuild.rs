use clap::Args;

#[derive(Debug, Args)]
#[command(ignore_errors = true)]
pub struct RebuildArgs {
    #[arg(help = "System name")]
    pub name: Option<String>,
    #[arg(short, long, help = "System architecture (eg: x86_64-linux)")]
    pub system: Option<String>,
}
