
pub mod appointment_type;
pub mod user;
pub mod room;
pub mod doctor_appointment_type;
pub mod room_appointment_type;
pub mod person;
pub mod doctor;
pub mod patient;
pub mod appointment;
pub mod available_appointment_resources;
pub mod possible_appointment;

pub trait Model<TPrimaryKey> {

    fn set_primary_key(&mut self, primary_key: &TPrimaryKey);

    fn get_primary_key(&self) -> &Option<TPrimaryKey>;

}
