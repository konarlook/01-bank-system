#[derive(Debug)]
pub enum ReadError {
    Validate(ValidationError),
    File(std::io::Error),
    IncorrectCSVHeader,
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadError::Validate(source) => {
                write!(f, "Read error: {}", source)
            }
            ReadError::File(_) => {
                write!(f, "Read file error")
            }
            ReadError::IncorrectCSVHeader => {
                write!(f, "Incorrect CSV header")
            }
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadError::Validate(source) => Some(source),
            ReadError::File(source) => Some(source),
            _ => None,
        }
    }
}

impl From<ValidationError> for ReadError {
    fn from(source: ValidationError) -> Self {
        ReadError::Validate(source)
    }
}

impl From<std::io::Error> for ReadError {
    fn from(source: std::io::Error) -> Self {
        ReadError::File(source)
    }
}

#[derive(Debug)]
pub enum ValidationError {
    UnknownKindType,
    UnknownCategoryType,
    NotFullData,
    InvalidAmount(std::num::ParseIntError),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValidationError::UnknownKindType => {
                write!(f, "Unknown type of kind")
            }
            ValidationError::UnknownCategoryType => {
                write!(f, "Unknown category type")
            }
            ValidationError::NotFullData => {
                write!(f, "Transaction contains too many or too less field")
            }
            ValidationError::InvalidAmount(source) => {
                write!(f, "Invalid amount: {}", source)
            }
        }
    }
}

impl std::error::Error for ValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ValidationError::InvalidAmount(source) => Some(source),
            _ => None,
        }
    }
}

impl From<std::num::ParseIntError> for ValidationError {
    fn from(source: std::num::ParseIntError) -> Self {
        ValidationError::InvalidAmount(source)
    }
}

#[derive(Debug)]
pub enum WriteError {
    AccessDenied,
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WriteError::AccessDenied => write!(f, "Write access denied"),
        }
    }
}

impl std::error::Error for WriteError {}
