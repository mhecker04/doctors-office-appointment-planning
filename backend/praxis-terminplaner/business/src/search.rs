use async_trait::async_trait;
use datalayer::{error::RepositoryError, room::RoomRepository, user::UserRepository, appointment_type::AppointmentTypeRepository};
use models::Model;

use crate::{base::Business, room::RoomBusiness, user::UserBusiness, appointment_type::AppointmentTypeBusiness};

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

pub async fn search_models(search_key: &String, search_clause: &String) -> Result<Vec<Box<dyn Model<String>>>, RepositoryError>{
    /* : Vec<Box<dyn Model<String>> */

    let models = match search_key.to_lowercase().as_str() {
        "user" => Ok(map_to_model_vec(UserBusiness{repository: UserRepository}.search(search_clause).await?)),
        "room" => Ok(map_to_model_vec(RoomBusiness{repository: RoomRepository}.search(search_clause).await?)),
        "appointmentType" => Ok(map_to_model_vec(AppointmentTypeBusiness{repository: AppointmentTypeRepository}.search(search_clause).await?)),
        _ => Err(RepositoryError::NoSearchBusinessFound)
    };

    models

}

fn map_to_model_vec<TModel>(models: Vec<TModel>) -> Vec<Box<dyn Model<String>>>
where
    TModel: Model<String> + 'static,
{

    let mut result = Vec::new();
    for model in models {
        
        let dynamic_model: Box<dyn Model<String>> = Box::new(model);

        result.push(dynamic_model);
    }

    result

}

pub mod errors {
    pub enum SearchError {
        NoSearchBusinessFound,
    }
}
