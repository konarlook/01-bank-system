use crate::error::{ReadError, ValidationError, WriteError};
use crate::model::{TxCategory, TxKind};

mod csv_format;
mod error;
mod model;
mod txt_format;

#[derive(Debug, Clone)]
pub struct Transaction {
    dt: String,
    category: TxCategory,
    kind: TxKind,
    amount: i64,
}

impl Transaction {
    pub fn from_string(delim: char, ops: &str) -> Result<Self, ValidationError> {
        let raw: Vec<&str> = ops
            .split(delim)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if raw.len() != 4 {
            return Err(ValidationError::NotFullData);
        }

        Ok(Self {
            dt: raw[0].to_string(),
            category: raw[1].parse()?,
            kind: raw[2].parse()?,
            amount: raw[3].parse()?,
        })
    }
}

trait Formater {
    fn read_from<R: std::io::Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError>;
    fn write_to<W: std::io::Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError>;
}
