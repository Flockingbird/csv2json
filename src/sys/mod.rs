use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

pub fn get_file_name(path: &AsRef<Path>) -> &str {
    path.as_ref()
        .file_stem()
        .expect("Could not get file name (err: file stem)")
        .to_str()
        .expect("Could not get file name (err: str)")
}

pub fn write_json_to_file(
    out_dir: &AsRef<Path>,
    file_name: &AsRef<Path>,
    data: &AsRef<[u8]>,
) -> Result<()> {
    let file_name = out_dir
        .as_ref()
        .join(file_name.as_ref())
        .with_extension("json");
    eprintln!("Writing to {}", file_name.to_string_lossy());
    let mut file = File::create(file_name)?;
    file.write(data.as_ref()).map(|_| ())
}
