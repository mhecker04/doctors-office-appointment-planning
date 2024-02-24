use serde::{Serialize, Deserialize};

use crate::{doctor::DoctorModel, room::RoomModel};

#[derive(Serialize, Deserialize)]
pub struct AvailableAppointmentRessourcesModel {
    pub doctors: Vec<DoctorModel>,
    pub rooms: Vec<RoomModel>
}
