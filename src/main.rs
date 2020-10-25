use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use clap::{crate_authors, crate_name, crate_version, App, Arg};
use zip::ZipArchive;

fn process_file<R: Read, P: AsRef<Path>>(
    reader: &mut R,
    output_path: P,
) -> Result<(), anyhow::Error> {
    let output_file = File::create(output_path)?;
    let num_threads = num_cpus::get();
    let mut encoder = zstd::Encoder::new(output_file, 0).with_context(|| "Creating encoder")?;

    encoder
        .multithread(num_threads as u32)
        .with_context(|| format!("Setting zstd thread count to {}", num_threads))?;

    io::copy(reader, &mut encoder)?;

    encoder.finish().with_context(|| "Writing end of zstd")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Extracts a zip archive with a single file and compresses it with zstd")
        .arg(
            Arg::with_name("zip")
                .required(true)
                .value_name("ZIP ARCHIVE"),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT FILE")
                .required(true),
        )
        .get_matches();

    let archive_path: PathBuf = matches.value_of("zip").unwrap().into();
    let output_path: PathBuf = matches.value_of("output").unwrap().into();

    let file = File::open(archive_path.as_path())
        .with_context(|| format!("Failed to open zip file `{:?}'", archive_path))?;

    let mut archive = ZipArchive::new(&file)?;

    if archive.len() != 1 {
        return Err(anyhow!("Expected zip archive to have exactly 1 file"));
    }

    let mut first_file = archive
        .by_index(0)
        .with_context(|| "Reading first entry in zip")?;

    process_file(&mut first_file, output_path.as_path())
        .with_context(|| format!("Writing to {:?}", output_path.as_path()))?;

    Ok(())
}
