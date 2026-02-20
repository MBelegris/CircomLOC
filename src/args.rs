use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "circom-analyzer", about = "Count lines of the circom files in a directory")]
pub struct Args {
    /// Path to the file or directory to analyze
    #[arg(short, long, default_value = "../")]
    pub circom_dir: String,
    /// Exclude files with the specified extension
    #[clap(short, long, value_parser)]
    pub exclude_ext: Option<String>,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}