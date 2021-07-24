use std::path::PathBuf;

use anyhow::{anyhow, Error};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Extracts a zip archive and converts it to a tar while compressing it with zstd"
)]
pub(crate) struct Opts {
    /// The input zip archive
    #[structopt(name = "ZIP ARCHIVE", required = true)]
    pub input: PathBuf,

    /// The output tar.zst archive
    #[structopt(name = "OUTPUT ARCHIVE", required = true)]
    pub output: PathBuf,

    /// Sets the zstd compression level (1-22) (0 means zstd default compression level)
    #[structopt(short = "l", long = "level", default_value = "0", parse(try_from_str = parse_compression_level))]
    pub compression_level: i32,
}

fn parse_compression_level(src: &str) -> Result<i32, Error> {
    let n: i32 = src.parse()?;

    if !(0..=22).contains(&n) {
        return Err(anyhow!("Compression level has to be in the range of 1-22"));
    }

    Ok(n)
}
