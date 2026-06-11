use crate::error::{ConvertError, ReadError, WriteError};
use crate::model::Transaction;

#[derive(Debug)]
pub enum Format {
    Text,
    CSV,
    Binary,
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Format::Text => write!(f, "txt"),
            Format::CSV => write!(f, "csv"),
            Format::Binary => write!(f, "bin"),
        }
    }
}

impl std::str::FromStr for Format {
    type Err = ConvertError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "txt" => Ok(Format::Text),
            "csv" => Ok(Format::CSV),
            "bin" => Ok(Format::Binary),
            _ => Err(ConvertError::UnknownFormat),
        }
    }
}

pub trait Formater {
    fn read_from<R: std::io::Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError>;
    fn write_to<W: std::io::Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError>;
}
