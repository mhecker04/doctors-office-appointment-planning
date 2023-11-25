

pub trait Repository<TEntity, TPrimaryKey> {

    fn insert(&self, entity: &TEntity) -> Result<usize, diesel::result::Error>;

    fn update(&self,entity: &TEntity)-> Result<usize, diesel::result::Error>;

    fn delete(&self, id: &TPrimaryKey)-> Result<usize, diesel::result::Error>;

    fn get(&self, id: &TPrimaryKey) -> Result<TEntity, diesel::result::Error>;

}

#[macro_export] 
macro_rules! delete {
    ($connection: expr, $table:expr, $id:expr) => {
        diesel::delete($table.filter($table.primary_key().eq($id)))
            .execute($connection)
    };
}

#[macro_export] 
macro_rules! insert_into { 
    ($connection: expr, $table:expr, $aggregate:expr) => {
        diesel::insert_into($table)
            .values($aggregate)
            .execute($connection)
    };
}

#[macro_export]
macro_rules! update {
    ($connection: expr, $table:expr, $aggregate:expr, $id: expr) => {
        diesel::update($table.find($id))
            .set($aggregate) 
            .execute($connection)
    };
}

#[macro_export]
macro_rules! get_by_id {
    ($connection: expr, $table:expr, $select: expr, $id: expr) => {

        $table 
            .filter($table.primary_key().eq($id))
            .select($select())
            .first($connection)

    };
}

#[macro_export]
macro_rules! fetch_entity {
    ($connection: expr, $table:expr, $select: expr, $predicate: expr) => {
        $table
            .filter($predicate)
            .select($select())
            .first($connection) 
    };
}

