

use datalayer::room::RoomRepository;
use models::room::RoomModel;

use crate::base::Business;

pub struct RoomBusiness {

    pub repository: RoomRepository

}

impl Business<RoomRepository, RoomModel, String> for RoomBusiness {


    fn get_repository(&self) -> &RoomRepository{
        &self.repository
    }


}
