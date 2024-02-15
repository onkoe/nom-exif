mod bbox;
mod error;
mod exif;
mod file;
mod heif;
mod jpeg;
mod mov;

pub use heif::parse_heif_exif;
pub use jpeg::parse_jpeg_exif;
pub use mov::parse_mov_metadata;

pub use exif::{ExifTag, IfdEntryValue};

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod testkit;
