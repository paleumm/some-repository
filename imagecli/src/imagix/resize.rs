use std::{
    fmt, fs, io,
    path::PathBuf,
    str::FromStr,
    time::{Duration, Instant},
};

use image::ImageFormat;

use super::error::ImagixError;

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}
impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
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

#[derive(Debug, PartialEq)]
pub enum Mode {
    Single,
    All,
}

impl FromStr for Mode {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError(
                "Wrong value for mode".to_string(),
            )),
        }
    }
}

pub fn get_image_files(src_dir: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    let entries = fs::read_dir(src_dir)
        .map_err(|_e| ImagixError::UserInputError("Invalid source folder".to_string()))?
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

fn resize_image(size: u32, src_dir: &mut PathBuf) -> Result<(), ImagixError> {
    let new_file_name = src_dir
        .file_stem()
        .unwrap()
        .to_str()
        .ok_or(std::io::ErrorKind::InvalidInput)
        .map(|f| format!("{}.png", f));

    let mut dest_dir = src_dir.clone();
    dest_dir.pop();
    dest_dir.push("tmp/");

    if !dest_dir.exists() {
        fs::create_dir(&dest_dir)?;
    }

    dest_dir.pop();
    dest_dir.push("tmp/tmp.png");
    dest_dir.set_file_name(new_file_name?.as_str());

    let timer = Instant::now();
    let img = image::open(&src_dir)?;
    let scaled = img.thumbnail(size, size);
    let mut output = fs::File::create(&dest_dir)?;
    scaled.write_to(&mut output, ImageFormat::Png)?;

    println!(
        "Thumbnailed file: {:?} to size {}x{} in {}. Output file in {:?}",
        src_dir,
        size,
        size,
        Elapsed::from(&timer),
        dest_dir
    );
    Ok(())
}

pub fn process_resize_request(
    size: SizeOption,
    mode: Mode,
    src_dir: &mut PathBuf,
) -> Result<(), ImagixError> {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };

    let _ = match mode {
        Mode::All => resize_all(size, src_dir)?,
        Mode::Single => resize_single(size, src_dir)?,
    };
    Ok(())
}

fn resize_single(size: u32, src_dir: &mut PathBuf) -> Result<(), ImagixError> {
    let mut src_dir = src_dir;
    resize_image(size, &mut src_dir)?;
    Ok(())
}

fn resize_all(size: u32, src_dir: &mut PathBuf) -> Result<(), ImagixError> {
    if let Ok(entries) = get_image_files(src_dir.to_path_buf()) {
        for mut entry in entries {
            resize_image(size, &mut entry)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_single_image_resize() {
        let mut path = PathBuf::from("/tmp/images/image1.jpg");
        let destination_path = PathBuf::from("/tmp/images/tmp/image1.png");
        match process_resize_request(SizeOption::Small, Mode::Single, &mut path) {
            Ok(_) => println!("Successful resize of single image"),
            Err(e) => println!("Error in single image: {:?}", e),
        }
        assert_eq!(true, destination_path.exists());
    }
    #[test]
    fn test_multiple_image_resize() {
        let mut path = PathBuf::from("/tmp/images/");
        let _res = process_resize_request(SizeOption::Small, Mode::All, &mut path);
        let destination_path1 = PathBuf::from("/tmp/images/tmp/image1.png");
        let destination_path2 = PathBuf::from("/tmp/images/tmp/image2.png");
        assert_eq!(true, destination_path1.exists());
        assert_eq!(true, destination_path2.exists());
    }
}
