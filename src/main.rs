use std::fs::File;
use std::path::Path;

use anyhow::Context;
use structopt::StructOpt;
use zip::ZipArchive;

mod cli;

fn process_zip_archive<P1: AsRef<Path>, P2: AsRef<Path>>(
    archive_path: P1,
    output_path: P2,
    compression_level: i32,
) -> Result<(), anyhow::Error> {
    let file = File::open(archive_path.as_ref())
        .with_context(|| format!("Failed to open zip file {:?}", archive_path.as_ref()))?;

    let mut zip = ZipArchive::new(&file)?;
    let output_file = File::create(output_path)?;
    let num_threads = num_cpus::get();

    // Create the zstandard encoder
    let mut encoder =
        zstd::Encoder::new(output_file, compression_level).with_context(|| "Creating encoder")?;
    encoder
        .multithread(num_threads as u32)
        .with_context(|| format!("Setting zstd thread count to {}", num_threads))?;

    let mut tar = tar::Builder::new(encoder);

    for file_index in 0..zip.len() {
        let zip_file = zip.by_index(file_index)?;

        if zip_file.is_dir() {
            continue;
        }

        let mut header = tar::Header::new_gnu();
        header.set_size(zip_file.size());
        header.set_mode(zip_file.unix_mode().unwrap_or(0o0644));
        header.set_mtime(zip_file.last_modified().to_time().to_timespec().sec as u64);
        header.set_uid(1000);
        header.set_gid(1000);
        header.set_cksum();

        let path = zip_file.name().to_string();
        tar.append_data(&mut header, path, zip_file)?;
    }

    tar.finish()?;

    tar.into_inner()?
        .finish()
        .with_context(|| "Finishing zstd")?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let opts = cli::Opts::from_args();
    let output_path = opts.output.as_path();

    process_zip_archive(opts.input, output_path, opts.compression_level)
        .with_context(|| format!("Writing to {:?}", output_path))?;

    Ok(())
}
