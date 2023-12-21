use async_trait::async_trait;
use models::Model;

use crate::error::RepositoryError;

#[async_trait]
pub trait SearchRepository<TModel, TPrimaryKey> 
    where TModel: Model<TPrimaryKey>
{

    async fn search(&self, search_clause: &String) -> Result<Vec<TModel>, RepositoryError>;

}