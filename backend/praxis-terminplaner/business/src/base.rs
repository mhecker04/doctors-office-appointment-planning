use async_trait::async_trait;
use datalayer::base::{Repository, RepositoryError};

#[async_trait]
pub trait Business<TRepository, TModel, TPrimaryKey>
where
    TRepository: Repository<TModel, TPrimaryKey>,
    TModel: Send + Sync,
    TPrimaryKey: Send + Sync
{
    fn get_repository(&self) -> &TRepository;

    async fn insert(&self, entity: &mut TModel) -> Result<TPrimaryKey, RepositoryError> {
        self.get_repository().create(entity).await
    }

    async fn update(&self, entity: &TModel) -> Result<(), RepositoryError> {
        self.get_repository().update(entity).await
    }

    async fn delete(&self, id: &TPrimaryKey) -> Result<(), RepositoryError> {
        self.get_repository().delete(id).await
    }

    async fn get_by_id(&self, id: &TPrimaryKey) -> Result<TModel, RepositoryError> {
        self.get_repository().get_by_id(id).await
    }
}
