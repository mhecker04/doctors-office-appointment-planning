use async_trait::async_trait;
use datalayer::error::RepositoryError;


#[async_trait]
pub trait SearchBusiness<TModel, TPriamryKey> {
    async fn search(&self, search_clause: &String) -> Result<Vec<TModel>, RepositoryError>;
}

#[macro_export]
macro_rules! implement_search_business {
    ($business:ty, $model:ty, $primary_key:ty) => {
        #[async_trait]
        impl SearchBusiness<$model, $primary_key> for $business {
            async fn search(&self, search_clause: &String) -> Result<Vec<$model>, RepositoryError> {
                self.get_repository().search(search_clause).await
            }
        }
    };
}

pub mod errors {
    pub enum SearchError {
        NoSearchBusinessFound,
    }
}
