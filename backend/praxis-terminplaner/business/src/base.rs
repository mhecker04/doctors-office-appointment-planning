
pub trait Business<TEntity, TPrimaryKey> {

    fn insert(&self, entity: &TEntity) -> Result<usize, diesel::result::Error>;

    fn update(&self,entity: &TEntity)-> Result<usize, diesel::result::Error>;

    fn delete(&self, id: &TPrimaryKey)-> Result<usize, diesel::result::Error>;

    fn get(&self, id: &TPrimaryKey) -> Result<TEntity, diesel::result::Error>;   

}