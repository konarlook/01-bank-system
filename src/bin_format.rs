use crate::error::{ReadError, WriteError};
use crate::format::Formater;
use crate::model::{Transaction, TxCategory, TxKind};
use std::io::{Read, Write};

pub struct BinFormater {}

impl Formater for BinFormater {
    fn read_from<R: Read>(r: &mut R) -> Result<Vec<Transaction>, ReadError> {
        let mut transactions: Vec<Transaction> = Vec::new();

        let mut magic_buf = [0u8; 4];
        r.read_exact(&mut magic_buf)?;
        if magic_buf != *Self::MAGIC_BYTES {
            return Err(ReadError::IncorrectMagicBytes);
        }

        let count = Self::read_u32(r)?;

        for _ in 0..count {
            transactions.push(Transaction {
                dt: Self::read_string(r)?,
                category: TxCategory::try_from(Self::read_u8(r)?)?,
                kind: TxKind::try_from(Self::read_u8(r)?)?,
                amount: Self::read_i64(r)?,
            });
        }

        Ok(transactions)
    }

    fn write_to<W: Write>(ts: &Vec<Transaction>, w: &mut W) -> Result<(), WriteError> {
        w.write_all(Self::MAGIC_BYTES)
            .map_err(WriteError::WriterIOError)?;
        let count: u32 = ts
            .len()
            .try_into()
            .map_err(|_| WriteError::TooManyTransactions)?;
        w.write_all(&count.to_le_bytes())?;
        for t in ts {
            let name_len: u32 =
                t.dt.len()
                    .try_into()
                    .map_err(|_| WriteError::TooManyTransactions)?;
            w.write_all(&name_len.to_le_bytes())?;

            w.write_all(&t.dt.as_bytes())?;
            w.write_all(&u8::from(t.category).to_le_bytes())?;
            w.write_all(&u8::from(t.kind).to_le_bytes())?;
            w.write_all(&t.amount.to_le_bytes())?;
        }
        Ok(())
    }
}

impl BinFormater {
    const MAGIC_BYTES: &'static [u8; 4] = b"TRNS";

    fn read_u32<R: Read>(r: &mut R) -> Result<u32, ReadError> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;

        Ok(u32::from_le_bytes(buf))
    }

    fn read_u8<R: Read>(r: &mut R) -> Result<u8, ReadError> {
        let mut buf = [0u8; 1];
        r.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    fn read_i64<R: Read>(r: &mut R) -> Result<i64, ReadError> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;

        Ok(i64::from_le_bytes(buf))
    }

    fn read_string<R: Read>(r: &mut R) -> Result<String, ReadError> {
        let lens = Self::read_u32(r)?;
        let mut vecs = vec![0u8; lens as usize];

        r.read_exact(&mut vecs)?;

        Ok(String::from_utf8(vecs)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::bin_format::BinFormater;
    use crate::format::Formater;
    use crate::model::{Transaction, TxCategory, TxKind};

    #[test]
    fn test_read_bin_format() {
        let ts = vec![
            Transaction {
                dt: "2026-12-01".to_string(),
                category: TxCategory::Salary,
                kind: TxKind::Income,
                amount: 100,
            },
            Transaction {
                dt: "1999-11-29".to_string(),
                category: TxCategory::Rent,
                kind: TxKind::Expense,
                amount: 100000,
            },
        ];

        let mut buf = Vec::new();
        let write = BinFormater::write_to(&ts, &mut buf);

        assert!(write.is_ok());

        let mut reader = buf.as_slice();
        let result = BinFormater::read_from(&mut reader);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ts);
    }

    #[test]
    fn test_write_bin_format() {
        let tx = Transaction {
            dt: "2026-04-01".to_string(),
            category: TxCategory::Salary,
            kind: TxKind::Income,
            amount: 120000,
        };

        let mut buffer = Vec::new();
        let w_res = BinFormater::write_to(&vec![tx], &mut buffer);
        assert!(w_res.is_ok());

        assert_eq!(&buffer[0..4], BinFormater::MAGIC_BYTES);
        assert_eq!(&buffer[4..8], &1u32.to_le_bytes());
        assert_eq!(&buffer[8..12], &10u32.to_le_bytes());

        assert_eq!(&buffer[12..22], b"2026-04-01");
        assert_eq!(&buffer[22..23], &[0]);
        assert_eq!(&buffer[23..24], &[0]);
        assert_eq!(&buffer[24..32], &120000i64.to_le_bytes());
        assert_eq!(buffer.len(), 32);
    }
}
