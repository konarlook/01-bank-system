use crate::error::{ReadError, WriteError};
use crate::model::{TxCategory, TxKind};

mod csv_format;
mod error;
mod model;
mod txt_format;

#[derive(Debug, Clone)]
pub struct Transaction{
    dt: String,
    category: TxCategory,
    kind: TxKind,
    amount: i64,
}

trait Formater {
    fn read_from<R: std::io::Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError>;
    fn write_to<W: std::io::Write>(t: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError>;
}
