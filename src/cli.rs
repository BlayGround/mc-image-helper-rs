use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::minecraft::LoaderType;

#[derive(Parser, Debug)]
#[command(name = "mc-image-helper", version = "1.0", about = "Minecraft Modpack and Plugin Helper")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    InstallModrinthModpack(InstallModrinthModpackArgs),
}

#[derive(Parser, Debug)]
pub struct InstallModrinthModpackArgs {
    /// Modrinth Project ID
    #[arg(short, long)]
    pub project: String,

    /// Output Directory
    #[arg(short = 'o', long = "output-directory", default_value = ".")]
    pub output_directory: PathBuf,

    /// Project Version
    #[arg(short, long, default_value = "latest")]
    pub version: String,

    /// Game Version
    #[arg(long, value_delimiter = ',')]
    pub game_version: Option<Vec<String>>,

    /// loader
    #[arg(short, long, value_enum, value_delimiter = ',')]
    pub loader: Option<Vec<LoaderType>>
}
