use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    #[clap(required = true, parse(from_os_str))]
    pub path: PathBuf,
}
