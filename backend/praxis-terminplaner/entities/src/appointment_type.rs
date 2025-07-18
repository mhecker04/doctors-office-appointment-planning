//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "appointment_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub appointment_type_id: String,
    pub appointment_type_name: Option<String>,
    pub duration: Option<Time>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::appointment::Entity")]
    Appointment,
    #[sea_orm(has_many = "super::doctor_appointment_type::Entity")]
    DoctorAppointmentType,
    #[sea_orm(has_many = "super::room_appointment_type::Entity")]
    RoomAppointmentType,
}

impl Related<super::appointment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Appointment.def()
    }
}

impl Related<super::doctor_appointment_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DoctorAppointmentType.def()
    }
}

impl Related<super::room_appointment_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoomAppointmentType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
