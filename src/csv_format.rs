use crate::error::{ReadError, ValidationError, WriteError};
use crate::{Formater, Transaction};
use std::io::{Read, Write};

pub struct CSVFormater {}

impl Formater for CSVFormater {
    fn read_from<R: Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError> {
        let mut st = String::new();
        r.read_to_string(&mut st)?;

        let mut raw: Vec<&str> = st
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let header = &raw.remove(0);
        if header.to_string() != "date,category,kind,amount" {
            return Err(ReadError::IncorrectCSVHeader);
        }

        let transactions: Vec<Transaction> =
            raw.into_iter()
                .map(|s| Self::parse(s.trim()))
                .collect::<Result<Vec<Transaction>, ValidationError>>()?;

        Ok(transactions)
    }

    fn write_to<W: Write>(t: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError> {
        todo!()
    }
}

impl CSVFormater {
    fn parse(ops: &str) -> Result<Transaction, ValidationError> {
        let raw: Vec<&str> = ops
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if raw.len() != 4 {
            return Err(ValidationError::NotFullData);
        };

        Ok(Transaction {
            dt: raw[0].to_string(),
            category: raw[1].parse()?,
            kind: raw[2].parse()?,
            amount: raw[3].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Formater;
    use crate::csv_format::CSVFormater;
    use crate::model::{TxCategory, TxKind};
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_read_csv_happy_path() {
        let data = r#"
            date,category,kind,amount
            2026-04-01,salary,income,120000
        "#;
        let cursor = Cursor::new(data);
        let mut reader = BufReader::new(cursor);
        let mut reader = BufReader::new(&mut reader);

        let result = CSVFormater::read_from(&mut reader);

        println!("{:?}", result);
        assert!(&result.is_ok());

        let tx = &result.unwrap()[0];

        assert_eq!(tx.dt, "2026-04-01");
        assert_eq!(tx.category, TxCategory::Salary);
        assert_eq!(tx.kind, TxKind::Income);
        assert_eq!(tx.amount, 120000);
    }
}
