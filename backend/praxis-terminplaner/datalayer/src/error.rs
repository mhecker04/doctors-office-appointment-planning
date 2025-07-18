
#[derive(Debug)]
pub enum RepositoryError {
    NoConnection,
    NoRecordFound,
    MappingError,
    NoPersonSpecified,
    InvalidDateTimeFormat,
    DateOutOfRange
}
