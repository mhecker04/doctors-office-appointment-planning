
use async_trait::async_trait;
use datalayer::room::RoomRepository;
use models::room::RoomModel;
use datalayer::error::RepositoryError;
use datalayer::search::SearchRepository;

use crate::{base::Business, search::SearchBusiness, implement_search_business};

pub struct RoomBusiness {

    pub repository: RoomRepository

}

impl Business<RoomRepository, RoomModel, String> for RoomBusiness {
    fn get_repository(&self) -> &RoomRepository{
        &self.repository
    }
}


impl RoomBusiness {
    pub async fn load_rooms_qualified_for_appointment_type(&self, appointment_type_id: &String) -> Result<Vec<RoomModel>, RepositoryError> {
        self.repository.load_rooms_qualified_for_appointment_type(appointment_type_id).await
    }
}

implement_search_business!(RoomBusiness, RoomModel, String);
