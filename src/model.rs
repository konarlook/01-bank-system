use crate::error::ValidationError;

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
