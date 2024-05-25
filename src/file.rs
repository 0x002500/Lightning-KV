use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write(key: String, value: String) -> Result<(), std::io::Error> {
    let path: &Path = Path::new(key.as_str());
    let mut file = File::create(path)?;
    file.write_all(value.as_bytes())?;

    Ok(())
}

