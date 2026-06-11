use crate::error::{ConvertError, ReadError, WriteError};
use crate::format::{Format, Formater};
use crate::model::Transaction;

mod bin_format;
mod csv_format;
pub mod error;
pub mod format;
mod model;
mod txt_format;

/// Варианты расхождения транзакций
#[derive(Debug, PartialEq)]
pub enum Difference {
    /// Расхождение списка транзакций по длине списка
    Length {
        left: usize,
        right: usize,
    },
    /// Расхождение транзакций по конкретному полю
    Field {
        position: usize,
        field: &'static str,
        left: String,
        right: String,
    },
    EmptyList,
}

impl std::fmt::Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Difference::Length {
                left: v_left,
                right: v_right,
            } => write!(
                f,
                "Transactions have different lengths (left: {}, right, {})",
                v_left, v_right
            ),
            Difference::Field {
                position: v_pos,
                field: v_field,
                left: v_left,
                right: v_right,
            } => write!(
                f,
                "Transaction have different field (pos: {}, field: {}, left: {}, right: {})",
                v_pos, v_field, v_left, v_right
            ),
            Difference::EmptyList => write!(f, "One of compare transactions is empty"),
        }
    }
}

/// Читает транзакции из любого источника, реализующего Read, в
/// формате format;
///
/// #Error
/// Возвращает 'ReadError', если произошла ошибка при чтении или при
/// валидации данных.
pub fn read_transactions<R: std::io::Read>(
    format: Format,
    r: &mut R,
) -> Result<Vec<Transaction>, ReadError> {
    match format {
        Format::Text => txt_format::TxTFormater::read_from(r),
        Format::CSV => csv_format::CSVFormater::read_from(r),
        Format::Binary => bin_format::BinFormater::read_from(r),
    }
}

/// Записывает список транзакций в любой приемник, реализующий Write,
/// в формат format;
///
/// #Error
/// Возвращает 'WriteError', если произошла ошибка записи транзакций в
/// файл.
pub fn write_transactions<W: std::io::Write>(
    format: Format,
    ts: &Vec<Transaction>,
    w: &mut W,
) -> Result<(), WriteError> {
    match format {
        Format::Text => txt_format::TxTFormater::write_to(ts, w),
        Format::CSV => csv_format::CSVFormater::write_to(ts, w),
        Format::Binary => bin_format::BinFormater::write_to(ts, w),
    }
}

/// Конвертирует поток транзакций из формата 'from' в формат 'to'
/// через общее представление транзакций.
///
/// #Error
/// Возвращает 'ConvertError', если произошла ошибка при конвертации (
/// ошибка чтения, записи или конвертации).
pub fn convert<R: std::io::Read, W: std::io::Write>(
    from: Format,
    to: Format,
    r: &mut R,
    w: &mut W,
) -> Result<(), ConvertError> {
    let ts = read_transactions(from, r)?;
    write_transactions(to, &ts, w)?;
    Ok(())
}

/// Сравнивает два списка поэлементно: возвращает разницу первого расхождения
/// или None, если списки совпадают.
pub fn compare(left: &Vec<Transaction>, right: &Vec<Transaction>) -> Option<Difference> {
    if left.len() == 0 || right.len() == 0 {
        return Some(Difference::EmptyList);
    }
    if left.len() != right.len() {
        return Some(Difference::Length {
            left: left.len(),
            right: right.len(),
        });
    }

    for (i, (left, right)) in left.iter().zip(right.iter()).enumerate() {
        if left.dt != right.dt {
            return Some(Difference::Field {
                position: i,
                field: "dt",
                left: left.dt.to_string(),
                right: right.dt.to_string(),
            });
        }
        if left.category != right.category {
            return Some(Difference::Field {
                position: i,
                field: "category",
                left: left.category.to_string(),
                right: right.category.to_string(),
            });
        }
        if left.kind != right.kind {
            return Some(Difference::Field {
                position: i,
                field: "kind",
                left: left.kind.to_string(),
                right: right.kind.to_string(),
            });
        }
        if left.amount != right.amount {
            return Some(Difference::Field {
                position: i,
                field: "amount",
                left: left.amount.to_string(),
                right: right.amount.to_string(),
            });
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{TxCategory, TxKind};
    use std::io::Cursor;

    fn sample() -> Vec<Transaction> {
        vec![
            Transaction {
                dt: "2026-01-01".to_string(),
                category: TxCategory::Salary,
                kind: TxKind::Income,
                amount: 120000,
            },
            Transaction {
                dt: "2026-01-02".to_string(),
                category: TxCategory::Rent,
                kind: TxKind::Expense,
                amount: 40000,
            },
        ]
    }

    #[test]
    fn test_read_transactions_dispatches_by_format() {
        let mut cursor = Cursor::new("2026-01-01;salary;income;120000\n");
        let ts = read_transactions(Format::Text, &mut cursor).unwrap();

        assert_eq!(ts.len(), 1);
        assert_eq!(ts[0].dt, "2026-01-01");
        assert_eq!(ts[0].category, TxCategory::Salary);
        assert_eq!(ts[0].kind, TxKind::Income);
        assert_eq!(ts[0].amount, 120000);
    }

    #[test]
    fn test_write_transactions_dispatches_by_format() {
        let mut out: Vec<u8> = Vec::new();

        write_transactions(Format::CSV, &sample(), &mut out).unwrap();

        let test = String::from_utf8(out).unwrap();
        assert_eq!(
            test,
            "date,category,kind,amount\n\
             2026-01-01,salary,income,120000\n\
             2026-01-02,rent,expense,40000\n"
        );
    }

    #[test]
    fn test_convert_csv_to_txt() {
        let input = "date,category,kind,amount\n2026-04-01,salary,income,120000\n";
        let mut reader = Cursor::new(input);
        let mut out: Vec<u8> = Vec::new();

        convert(Format::CSV, Format::Text, &mut reader, &mut out).unwrap();

        assert_eq!(
            String::from_utf8(out).unwrap(),
            "2026-04-01;salary;income;120000\n"
        );
    }

    #[test]
    fn test_convert_propagates_read_error() {
        let mut reader = Cursor::new("totally,not,a,header\n2026-04-01,salary,income,1\n");
        let mut out: Vec<u8> = Vec::new();

        let err = convert(Format::CSV, Format::Text, &mut reader, &mut out).unwrap_err();

        assert!(matches!(err, ConvertError::ReadError(_)));
        assert!(out.is_empty())
    }

    #[test]
    fn test_compare_equal_list() {
        assert_eq!(compare(&sample(), &sample()), None)
    }

    #[test]
    fn test_compere_first_differing_field() {
        let left = sample();
        let mut right = sample();
        right[1].amount = 7000;

        assert_eq!(
            compare(&left, &right),
            Some(Difference::Field {
                position: 1,
                field: "amount",
                left: "40000".to_string(),
                right: "7000".to_string(),
            })
        );
    }

    #[test]
    fn test_compare_reports_length_mismatch() {
        let left = sample();
        let mut right = sample();
        right.pop();

        assert_eq!(
            compare(&left, &right),
            Some(Difference::Length { left: 2, right: 1 })
        );
    }

    #[test]
    fn test_compare_field_priority_within_one_transaction() {
        let left = sample();
        let mut right = sample();
        right[0].dt = "1999-01-01".to_string();
        right[0].amount = 1;

        let diff = compare(&left, &right).unwrap();
        assert!(matches!(
            diff,
            Difference::Field {
                field: "dt",
                position: 0,
                ..
            }
        ))
    }

    #[test]
    fn test_compare_position_zero_based() {
        let left = sample();
        let right: Vec<Transaction> = vec![];

        assert_eq!(compare(&left, &right), Some(Difference::EmptyList))
    }
}
