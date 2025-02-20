use std::fs::File;
use std::io::Write;

pub async fn write_data(key: String, value: String) -> std::io::Result<()> {
    let mut buffer: File = File::create(key)?;
    buffer.write_all(value.as_bytes())?;

    buffer.sync_all()?;

    Ok(())
}
