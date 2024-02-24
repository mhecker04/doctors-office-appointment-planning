pub mod doctor;

pub trait PrefetchPath<TModel, TRelatedEntity>
    where TRelatedEntity: EntityTrait,
{

    fn get_related_entity(&self) -> TRelatedEntity;

    fn set_in_model(&self, model: TModel, related_model: TRelatedEntity::Model);

}
