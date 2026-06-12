use crate::error::{ReadError, WriteError};
use crate::format::Formater;
use crate::model::Transaction;
use std::io::{BufRead, BufReader, Read, Write};

pub struct TxTFormater {}

impl Formater for TxTFormater {
    fn read_from<R: Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError> {
        let reader = BufReader::new(r);
        let mut tx: Vec<Transaction> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            tx.push(Transaction::from_string(';', line)?);
        }
        Ok(tx)
    }

    fn write_to<W: Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError> {
        for t in ts {
            writeln!(w, "{};{};{};{}", t.dt, t.category, t.kind, t.amount)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Formater;
    use crate::model::{Transaction, TxCategory, TxKind};
    use crate::txt_format::TxTFormater;
    use std::io::{BufRead, BufReader, BufWriter, Cursor, Write};

    #[test]
    fn test_read_txt_happy_path() {
        let data = r#"
            2026-04-01;salary;income;120000
        "#;

        let cursor = Cursor::new(data);
        let mut reader = BufReader::new(cursor);
        let result = TxTFormater::read_from(&mut reader);

        assert!(&result.is_ok());

        let tx = &result.unwrap()[0];

        assert_eq!(tx.dt, "2026-04-01");
        assert_eq!(tx.category, TxCategory::Salary);
        assert_eq!(tx.kind, TxKind::Income);
        assert_eq!(tx.amount, 120000);
    }

    #[test]
    fn test_write_txt_happy_path() {
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
            let w_res = TxTFormater::write_to(&vec![tx], &mut writer);

            assert!(w_res.is_ok());

            writer.flush().expect("test flush error");
        }

        cursor.set_position(0);
        let lines: Vec<String> = BufReader::new(cursor).lines().flatten().collect();

        assert_eq!(lines[0], "2026-04-01;salary;income;120000")
    }
}
