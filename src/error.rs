#[derive(Debug)]
pub enum ReadError {
    Validate(ValidationError),
    File(std::io::Error),
    IncorrectCSVHeader,
    IncorrectMagicBytes,
    InvalidTimeConvert(std::string::FromUtf8Error),
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
            ReadError::IncorrectMagicBytes => {
                write!(f, "Incorrect magic bytes")
            }
            ReadError::InvalidTimeConvert(source) => {
                write!(f, "Invalid time conversion: {}", source)
            }
        }
    }
}

impl std::error::Error for ReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadError::Validate(source) => Some(source),
            ReadError::File(source) => Some(source),
            ReadError::InvalidTimeConvert(source) => Some(source),
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

impl From<std::string::FromUtf8Error> for ReadError {
    fn from(source: std::string::FromUtf8Error) -> Self {
        ReadError::InvalidTimeConvert(source)
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
    WriterIOError(std::io::Error),
    TooManyTransactions,
    TooLargeDateString,
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WriteError::WriterIOError(err) => {
                write!(f, "Write error: {}", err)
            }
            WriteError::TooManyTransactions => {
                write!(f, "File contains too many transactions")
            }
            WriteError::TooLargeDateString => {
                write!(f, "Transaction contains too large date string")
            }
        }
    }
}

impl std::error::Error for WriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WriteError::WriterIOError(source) => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for WriteError {
    fn from(source: std::io::Error) -> Self {
        WriteError::WriterIOError(source)
    }
}
