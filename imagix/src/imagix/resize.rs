use std::{fmt, fs, io};
use std::fmt::write;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, Instant};
use image::ImageFormat;

use super::error::ImagixError;

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

pub enum SizeOption {
    Small,
    Medium,
    Large,
}

pub enum Mode {
    Single,
    All,
}

pub fn process_resize_request(
    size: SizeOption,
    mode: Mode,
    src_folder: &mut PathBuf,
) -> Result<(), ImagixError> {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };
    let _ = match mode {
        Mode::All => resize_all(size, src_folder)?,
        Mode::Single => resize_single(size, src_folder)?,
    };
    Ok(())
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} us", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

impl FromStr for SizeOption {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small),
        }
    }
}

impl FromStr for Mode {
    type Err = ImagixError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError(
                "Wrong value for mode.".to_string(),
            )),
        }
    }
}

pub fn get_image_files(src_folder: PathBuf) ->
Result<Vec<PathBuf>, ImagixError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|e| ImagixError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .filter(|r| {
            r.extension() == Some("JPG".as_ref())
            || r.extension() == Some("jpg".as_ref())
            || r.extension() == Some("PNG".as_ref())
            || r.extension() == Some("png".as_ref())
        })
        .collect();
    Ok(entries)
}

fn resize_single(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    let mut src_folder  = src_folder;
    resize_image(size, &mut src_folder)?;
    Ok(())
}

fn resize_all(size: u32, src_folder: &mut PathBuf) -> Result<(), ImagixError> {
    if let Ok(entries) = get_image_files(src_folder.to_path_buf()) {
        for mut entry in entries {
            resize_single(size, &mut entry)?;
        }
    };
    Ok(())
}

fn resize_image(size: u32, src_folder: &mut PathBuf) ->
Result<(), ImagixError> {
    let new_file_name = src_folder
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));
    //构造目标文件夹
    let mut desc_folder = src_folder.clone();
    desc_folder.pop();
    desc_folder.push("tmp/");
    if !desc_folder.exists() {
        fs::create_dir(&desc_folder)?;
    }
    desc_folder.pop();
    desc_folder.push("tmp/tmp.png");
    desc_folder.set_file_name(new_file_name?.as_str());

    let timer = Instant::now();
    let img = image::open(&src_folder)?;
    let scaled = img.thumbnail(size, size);
    let mut output = fs::File::create(&desc_folder)?;
    scaled.write_to(&mut output, ImageFormat::Png)?;
    println!(
        "Thumbnail file: {:?} to size {}x{} in {}. Output file in {:?}",
        src_folder,
        size,
        size,
        Elapsed::from(&timer),
        desc_folder
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_image_resize() {
        let mut path = PathBuf::from(
            "/Users/nino/CLionProjects/Rust_System/imagix/image1.jpg");
        let des_path = PathBuf::from(
            "/Users/nino/CLionProjects/Rust_System/imagix/tmp/image1.png");

        match process_resize_request(SizeOption::Small, Mode::Single, &mut path) {
            Ok(_) => println!("Success."),
            Err(e) => println!("Failed. {}", e),
        }
        assert_eq!(true, des_path.exists());
    }
}