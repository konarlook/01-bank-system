use crate::error::{ReadError, WriteError};
use crate::model::Transaction;

pub trait Formater {
    fn read_from<R: std::io::Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError>;
    fn write_to<W: std::io::Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError>;
}
