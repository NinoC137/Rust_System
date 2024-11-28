use std::path::PathBuf;
use super::error::ImagixError;
use super::resize::get_image_files;

pub fn get_stats(src_folder: PathBuf) -> Result<(usize, f64), ImagixError> {
    let image_files = get_image_files
        (src_folder.to_path_buf())?;
    let size = image_files
        .iter()
        .map(move |file| file.metadata().unwrap().len())
        .sum::<u64>();
    Ok((image_files.len(), (size / 100_000_0) as f64))
}
