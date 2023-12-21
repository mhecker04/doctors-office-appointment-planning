
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

implement_search_business!(RoomBusiness, RoomModel, String);
