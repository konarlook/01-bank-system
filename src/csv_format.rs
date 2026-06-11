use crate::error::{ReadError, ValidationError, WriteError};
use crate::format::Formater;
use crate::model::Transaction;
use std::io::{Read, Write};

const CSV_HEADER: &str = "date,category,kind,amount";

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
        if header.to_string() != CSV_HEADER {
            return Err(ReadError::IncorrectCSVHeader);
        }

        let transactions: Vec<Transaction> = raw
            .into_iter()
            .map(|s| Transaction::from_string(',', s))
            .collect::<Result<Vec<Transaction>, ValidationError>>()?;

        Ok(transactions)
    }

    fn write_to<W: Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError> {
        writeln!(w, "{}", CSV_HEADER)?;
        for t in ts {
            writeln!(w, "{},{},{},{}", t.dt, t.category, t.kind, t.amount)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::csv_format::CSVFormater;
    use crate::format::Formater;
    use crate::model::{Transaction, TxCategory, TxKind};
    use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};

    #[test]
    fn test_read_csv_happy_path() {
        let data = r#"
            date,category,kind,amount
            2026-04-01,salary,income,120000
        "#;
        let cursor = Cursor::new(data);
        let mut reader = BufReader::new(cursor);

        let result = CSVFormater::read_from(&mut reader);

        assert!(&result.is_ok());

        let tx = &result.unwrap()[0];

        assert_eq!(tx.dt, "2026-04-01");
        assert_eq!(tx.category, TxCategory::Salary);
        assert_eq!(tx.kind, TxKind::Income);
        assert_eq!(tx.amount, 120000);
    }

    #[test]
    fn test_write_csv_happy_path() {
        let tx = Transaction {
            dt: "2026-04-01".to_string(),
            category: TxCategory::Salary,
            kind: TxKind::Income,
            amount: 120000,
        };

        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            let w_res = CSVFormater::write_to(&vec![tx], &mut writer);

            assert!(w_res.is_ok());

            writer.flush().expect("test flush error");
        }

        cursor.set_position(0);
        let lines: Vec<String> = BufReader::new(cursor).lines().flatten().collect();

        assert_eq!(lines[0], "date,category,kind,amount");
        assert_eq!(lines[1], "2026-04-01,salary,income,120000");
    }
}
