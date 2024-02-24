use async_trait::async_trait;
use datalayer::base::{ListRepository, Repository};
use datalayer::error::RepositoryError;
use models::Model;

#[async_trait]
pub trait Business<TRepository, TModel, TPrimaryKey>
where
    TRepository: Repository<TModel, TPrimaryKey>,
    TModel: Send + Sync + Model<TPrimaryKey>,
    TPrimaryKey: Send + Sync,
{
    fn get_repository(&self) -> &TRepository;

    async fn create(&self, model: &mut TModel) -> Result<TPrimaryKey, RepositoryError> {
        self.get_repository().create(model).await
    }

    async fn update(&self, entity: &TModel) -> Result<(), RepositoryError> {
        self.get_repository().update(entity).await
    }

    async fn delete(&self, id: &TPrimaryKey) -> Result<TPrimaryKey, RepositoryError> {
        self.get_repository().delete(id).await
    }

    async fn get_by_id(&self, id: &TPrimaryKey) -> Result<TModel, RepositoryError> {
        self.get_repository().get_by_id(id).await
    }
}

#[async_trait]
pub trait ListBusiness<TRepository, TModel, TPrimaryKey>:
    Business<TRepository, TModel, TPrimaryKey>
where
    TRepository: Repository<TModel, TPrimaryKey>
        + ListRepository<TModel, TPrimaryKey>,
    TModel: Send + Sync + Model<TPrimaryKey>,
    TPrimaryKey: Send + Sync,
{

    async fn get_by_parent_id(&self, id: &TPrimaryKey) -> Result<Vec<TModel>, RepositoryError> {
        self.get_repository().get_by_parent_id(id).await
    }

}
