use crate::error::ValidationError;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub dt: String,
    pub category: TxCategory,
    pub kind: TxKind,
    pub amount: i64,
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TxKind {
    Income,
    Expense,
}

impl std::fmt::Display for TxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TxKind::Income => write!(f, "income"),
            TxKind::Expense => write!(f, "expense"),
        }
    }
}

impl std::str::FromStr for TxKind {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "income" => Ok(TxKind::Income),
            "expense" => Ok(TxKind::Expense),
            _ => Err(ValidationError::UnknownKindType),
        }
    }
}

impl TryFrom<u8> for TxKind {
    type Error = ValidationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TxKind::Income),
            1 => Ok(TxKind::Expense),
            _ => Err(ValidationError::UnknownKindType),
        }
    }
}

impl From<TxKind> for u8 {
    fn from(kind: TxKind) -> Self {
        match kind {
            TxKind::Income => 0,
            TxKind::Expense => 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TxCategory {
    Salary,
    Rent,
}

impl std::fmt::Display for TxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TxCategory::Salary => write!(f, "salary"),
            TxCategory::Rent => write!(f, "rent"),
        }
    }
}

impl std::str::FromStr for TxCategory {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "salary" => Ok(TxCategory::Salary),
            "rent" => Ok(TxCategory::Rent),
            _ => Err(ValidationError::UnknownCategoryType),
        }
    }
}

impl TryFrom<u8> for TxCategory {
    type Error = ValidationError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TxCategory::Salary),
            1 => Ok(TxCategory::Rent),
            _ => Err(ValidationError::UnknownCategoryType),
        }
    }
}

impl From<TxCategory> for u8 {
    fn from(kind: TxCategory) -> Self {
        match kind {
            TxCategory::Salary => 0,
            TxCategory::Rent => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ValidationError;
    use crate::model::{TxCategory, TxKind};

    #[test]
    fn test_convert_tx_category_to_bytes() {
        assert_eq!(u8::from(TxCategory::Salary), 0);
        assert_eq!(u8::from(TxCategory::Rent), 1);

        assert_eq!(
            TxCategory::try_from(u8::from(TxCategory::Salary)).unwrap(),
            TxCategory::Salary
        );
        assert_eq!(
            TxCategory::try_from(u8::from(TxCategory::Rent)).unwrap(),
            TxCategory::Rent
        );

        assert!(matches!(
            TxCategory::try_from(7),
            Err(ValidationError::UnknownCategoryType)
        ));
    }

    #[test]
    fn test_convert_tx_kind_from_bytes() {
        assert_eq!(u8::from(TxKind::Income), 0);
        assert_eq!(u8::from(TxKind::Expense), 1);

        assert_eq!(
            TxKind::try_from(u8::from(TxKind::Income)).unwrap(),
            TxKind::Income
        );
        assert_eq!(
            TxKind::try_from(u8::from(TxKind::Expense)).unwrap(),
            TxKind::Expense
        );
        assert!(matches!(
            TxKind::try_from(7),
            Err(ValidationError::UnknownKindType)
        ));
    }
}
