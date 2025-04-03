use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    pub pattern: String,
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
}
