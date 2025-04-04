use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_to_file(message: String, log_file: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(log_entry.as_byte())?;
    writer.flush()?;
    Ok(())
}
