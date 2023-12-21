
pub mod appointment_type;
pub mod user;
pub mod room;
pub mod doctor_appointment_type;

pub trait Model<TPrimaryKey> {

    fn set_primary_key(&mut self, primary_key: &TPrimaryKey);

    fn get_primary_key(&self) -> &Option<TPrimaryKey>;

}