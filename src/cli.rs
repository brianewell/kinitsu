pub use clap::Parser;
pub use std::path::PathBuf;

#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
pub struct Cli {
    /// The set of directories to operate on
    #[arg(default_value=".")]
    paths: Vec<PathBuf>,

    /// Path to the global configuration file
    #[arg(short='C',long,default_value="/etc/kinitsu.toml")]
    config: PathBuf,

    /// Per-directory configuration filename
    #[arg(short='c',long,default_value=".kinitsu.{type}")]
    directory_config: PathBuf,

    /// Comma-separated ordered list of configuration file formats to check
    #[arg(short,long,default_value="toml,yaml,json,xml")]
    types: String,

    /// Recursively traverse all child directories within the set of provided directory paths
    #[arg(short,long)]
    recursive: bool,

    /// Follow symbolic links while traversing directories
    #[arg(short,long)]
    symlinks: bool,

    /// Run as a background process, continually scanning directories for changes
    #[arg(short,long)]
    daemonize: bool,

    /// Enable interactive mode, showing a summary of changes and prompting users for confirmation of operations on each file
    #[arg(short,long)]
    interactive: bool,

    /// Preserve original files when performing destructive operations; specifically avoids removing source files and overwriting conflicting destination files
    #[arg(short,long)]
    preserve: bool,

    /// Perform a dry-run ("No-op"); no files will be modified. This is useful in conjunction with the -v option to validate that the expected operations will be performed
    #[arg(short,long)]
    noop: bool,

    /// Produce output in JSON format for scripting or integration with other tools
    #[arg(short,long)]
    json: bool,

    /// Suppress all non-critical output
    #[arg(short,long)]
    quiet: bool,

    /// Increase verbosity of output
    #[arg(short,long)]
    verbose: bool
}
