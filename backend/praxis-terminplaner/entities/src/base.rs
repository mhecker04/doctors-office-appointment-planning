
pub trait NewEntity<TPrimaryKey> {
    fn set_primary_key(&mut self, primary_key: TPrimaryKey);
}