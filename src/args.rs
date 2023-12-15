use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
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
}
