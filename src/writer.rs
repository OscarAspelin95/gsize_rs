use crate::errors::AppError;
use bio_utils_rs::io::get_bufwriter;
use std::path::PathBuf;

pub fn write_histogram(outfile: PathBuf, hist: &[usize]) -> Result<(), AppError> {
    let mut writer = get_bufwriter(Some(outfile))?;
    writer.write_all(b"count\tfrequency\n")?;

    for (count, frequency) in hist.iter().enumerate().skip(1) {
        writer.write_all(format!("{}\t{}\n", count, frequency).as_bytes())?;
    }

    writer.flush()?;

    Ok(())
}
