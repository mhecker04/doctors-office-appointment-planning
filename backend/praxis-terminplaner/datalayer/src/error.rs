
#[derive(Debug)]
pub enum RepositoryError {
    NoConnection,
    NoRecordFound,
    MappingError,
    NoSearchBusinessFound
}
