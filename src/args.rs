
use clap::Parser;


#[derive(Debug, Parser)]
#[clap(
    name = "rustyvibes",
    version,
    about = "A Rust CLI that makes mechanical keyboard sound effects on every key press"
)]
#[clap(propagate_version = true)]
pub struct ArgParser {
    
    /// The path of the soundpack
    pub soundpack: String,

    /// The volume to be set. 
    /// Default: 100
    #[arg(short, long)]
    pub volume: Option<u16>
}
