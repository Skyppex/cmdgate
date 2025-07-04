use clap::Parser;

/// this is a simple tool to run cmd commands
/// from a file or stdin and write the output to a file or stdout
#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct Args {
    /// Path to file to read from, otherwise stdin
    #[arg(short, long)]
    pub source: Option<String>,

    /// Path to file to write to, otherwise stdout
    #[arg(short, long)]
    pub destination: Option<String>,

    /// cmd command to run
    #[arg(short, long)]
    pub command: String,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,

    /// Interpret the output from the --command as utf8 bytes
    #[arg(short, long)]
    pub utf8: bool,
}
