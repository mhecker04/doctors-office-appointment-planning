use async_trait::async_trait;
use chrono::{NaiveDateTime, TimeDelta};
use datalayer::{
    appointment::{add_naive_time_to_naive_date_time, AppointmentRepository},
    error::RepositoryError,
};
use models::{
    appointment::AppointmentModel,
    available_appointment_resources::AvailableAppointmentRessourcesModel, doctor::DoctorModel,
    possible_appointment::PosssibleAppointmentModel, room::RoomModel,
};

use crate::{
    appointment_type::AppointmentTypeBusiness, base::Business, doctor::DoctorBusiness,
    room::RoomBusiness,
};

pub struct AppointmentBusiness {
    pub repository: AppointmentRepository,
    pub appointment_type_business: AppointmentTypeBusiness,
    pub room_business: RoomBusiness,
    pub doctor_business: DoctorBusiness,
}

#[async_trait]
impl Business<AppointmentRepository, AppointmentModel, String> for AppointmentBusiness {
    fn get_repository(&self) -> &AppointmentRepository {
        &self.repository
    }
}

const MAX_APPOINTMENT_COUNT: usize = 200;

impl AppointmentBusiness {
    pub async fn get_available_ressources(
        &self,
        appointment_type_id: &String,
        datetime: NaiveDateTime,
    ) -> Result<AvailableAppointmentRessourcesModel, RepositoryError> {
        let appointment_type = self
            .appointment_type_business
            .get_by_id(appointment_type_id)
            .await?;

        self.repository
            .get_available_ressources(datetime, &appointment_type)
            .await
    }

    pub async fn get_possible_appointments(
        &self,
        appointment_type_id: &String,
        start_date_time: NaiveDateTime,
        end_date_time: NaiveDateTime,
    ) -> Result<Vec<PosssibleAppointmentModel>, RepositoryError> {

        let conflicting_appointments = self
            .repository
            .get_conflicting_appointments(appointment_type_id)
            .await?;

        let appointment_type = self
            .appointment_type_business
            .get_by_id(appointment_type_id)
            .await?;

        let mut date_time_to_check = start_date_time.clone();

        let mut possible_appointments: Vec<PosssibleAppointmentModel> = Vec::new();

        let allowed_rooms_for_appointment_type = self
            .room_business
            .load_rooms_qualified_for_appointment_type(appointment_type_id)
            .await?;

        let allowed_doctors_for_appointment_type = self
            .doctor_business
            .load_doctors_qualified_for_appointment_type(appointment_type_id)
            .await?;

        while possible_appointments.len() < MAX_APPOINTMENT_COUNT {
            let appointment_start_time = date_time_to_check.clone();
            let appointment_end_time = add_naive_time_to_naive_date_time(
                &appointment_start_time,
                &appointment_type.duration,
            );

            let available_doctors: Vec<&DoctorModel> = allowed_doctors_for_appointment_type
                .iter()
                .filter(|doctor| {
                    !conflicting_appointments.iter().any(|appointment| {
                        appointment.end_date_time >= appointment_start_time
                            && appointment.start_date_time <= appointment_end_time
                            && Some(&appointment.doctor_id) == doctor.doctor_id.as_ref()
                    })
                })
                .collect();

            let available_rooms: Vec<&RoomModel> = allowed_rooms_for_appointment_type
                .iter()
                .filter(|room| {
                    !conflicting_appointments.iter().any(|appointment| {
                        appointment.end_date_time >= appointment_start_time
                            && appointment.start_date_time <= appointment_end_time
                            && appointment.room_id == room.room_id
                    })
                })
                .collect();

            if available_rooms.len() > 0 && available_doctors.len() > 0 {
                let possible_appointment_model = PosssibleAppointmentModel {
                    doctors: available_doctors.into_iter().cloned().collect(),
                    rooms: available_rooms.into_iter().cloned().collect(),
                    appointment_type_id: appointment_type_id.clone(),
                    from: appointment_start_time,
                    to: appointment_end_time,
                };

                possible_appointments.push(possible_appointment_model);
            }

            match date_time_to_check.checked_add_signed(TimeDelta::minutes(30)) {
                Some(new_date_time_to_check) => {
                    date_time_to_check = new_date_time_to_check;
                }
                None => {
                    return Err(RepositoryError::DateOutOfRange);
                }
            }
        }

        Ok(possible_appointments)
    }
}
