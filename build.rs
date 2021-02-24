use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src");

    let src_dir = "src";
    let dest = "output/src.zip";

    if !Path::new("output").exists() {
        std::fs::create_dir("output")?;
    }

    let path = Path::new(dest);
    let file = File::create(&path)?;

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        file,
        zip::CompressionMethod::Deflated,
    )?;

    Ok(())
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default().compression_method(method);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path;

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_str().unwrap(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().len() != 0 {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory(name.to_str().unwrap(), options)?;
        }
    }

    // Add Cargo.toml as it resides outside of the src directory
    let cargo_path = "Cargo.toml";
    zip.start_file(cargo_path, options)?;
    let mut f = File::open(cargo_path)?;

    f.read_to_end(&mut buffer)?;
    zip.write_all(&*buffer)?;

    zip.finish()?;
    Result::Ok(())
}
