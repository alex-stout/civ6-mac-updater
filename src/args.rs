use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "Civ 6 Mac Version Updater")]
#[command(author = "Alex Stout")]
#[command(version = "0.1")]
#[command(about = "Updates Civ 6 Mac version to match Windows.", long_about = None)]
pub struct CivUpdaterArgs {
    /// Version number with the format like "1.0.12.31"
    #[arg(long, required = false, default_value = "")]
    pub version_number: String,

    /// Version number with the format like "(859676)"
    #[arg(long, default_value = "")]
    pub version_str: String,
}
