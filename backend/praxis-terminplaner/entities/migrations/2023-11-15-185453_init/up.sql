CREATE TABLE IF NOT EXISTS `user`
(   
    user_id char(36),
    username varchar(32),
    `password` char(72),
    CONSTRAINT PK_User PRIMARY KEY(user_id)
);

CREATE TABLE IF NOT EXISTS role 
( 
    role_id CHAR(36), 
    role_name varchar(64), 
    CONSTRAINT PK_Role PRIMARY KEY (role_id) 
); 

CREATE TABLE IF NOT EXISTS permission 
( 
    permission_id CHAR(36), 
    permission_name varchar(64), 
    web_url varchar(128), 
    parent_permission_id CHAR(36), 
    CONSTRAINT PK_Permission PRIMARY KEY(permission_id), 
    CONSTRAINT FK_Permission_ParentPermission FOREIGN KEY (parent_permission_id) REFERENCES permission(permission_id) 
); 

CREATE TABLE IF NOT EXISTS user_permission
(
    user_permission_id CHAR(36),
    user_id CHAR(36),
    permission_id CHAR(36),
    CONSTRAINT PK_UserPermission PRIMARY KEY(user_permission_id),
    CONSTRAINT FK_UserPermission_User FOREIGN KEY (user_id) REFERENCES `user`(user_id),
    CONSTRAINT FK_UserPermission_Permission FOREIGN KEY (permission_id) REFERENCES permission(permission_id)
);

CREATE TABLE IF NOT EXISTS role_permission 
( 
    role_permission_id CHAR(36), 
    role_id CHAR(36), 
    permission_id CHAR(36), 
    CONSTRAINT PK_RolePermission PRIMARY KEY(role_permission_id), 
    CONSTRAINT FK_RolePermission_Role FOREIGN KEY (role_id) REFERENCES role(role_id), 
    CONSTRAINT FK_RolePermission_Permission FOREIGN KEY (permission_id) REFERENCES permission(permission_id) 
); 

CREATE TABLE IF NOT EXISTS appointment_type
(
    appointment_type_id CHAR(36),
    appointment_type_name VARCHAR(64),
    duration time,
    CONSTRAINT PK_AppointmentType PRIMARY KEY (appointment_type_id)
);  

CREATE TABLE IF NOT EXISTS room (
    room_id char(36),
    room_name varchar(32),
    room_number decimal(5,0),
    CONSTRAINT PK_Room PRIMARY KEY (room_id)
);

CREATE TABLE IF NOT EXISTS room_appointment_type
(
    room_appointment_type_id CHAR(36),
    room_id CHAR(36),
    appointment_type_id CHAR(36),
    CONSTRAINT PK_RoomAppointmentType PRIMARY KEY(room_appointment_type_id),
    CONSTRAINT FK_RoomAppointmentType_Room FOREIGN KEY (room_id) REFERENCES room(room_id),
    CONSTRAINT FK_RoomAppointmentType_AppointmentType FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id)
);

CREATE TABLE IF NOT EXISTS doctor
(
    doctor_id CHAR(36),
    user_id CHAR(36),
    CONSTRAINT PK_Doctor PRIMARY KEY(doctor_id),
    CONSTRAINT FK_Doctor_User FOREIGN KEY (user_id) REFERENCES `user`(user_id)
);

CREATE TABLE IF NOT EXISTS doctor_appointment_type
(
    doctor_appointment_type_id CHAR(36),
    appointment_type_id CHAR(36),
    doctor_id CHAR(36),
    CONSTRAINT PK_DoctorAppointmentType PRIMARY KEY(doctor_appointment_type_id),
    CONSTRAINT FK_DoctorAppointmentType_AppointmentType FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id),
    CONSTRAINT FK_DoctorAppointmentType_Doctor FOREIGN KEY (doctor_id) REFERENCES doctor(doctor_id)
);

CREATE TABLE IF NOT EXISTS patient
(
    patient_id CHAR(36),
    user_id CHAR(36),
    CONSTRAINT PK_Patient PRIMARY KEY (patient_id),
    CONSTRAINT FK_Patient_User FOREIGN KEY (user_id) REFERENCES `user`(user_id)
);

CREATE TABLE IF NOT EXISTS appointment
(
    appointment_id CHAR(36),
    appointment_type_id CHAR(36),
    doctor_id CHAR(36),
    `from` datetime,
    patient_id CHAR(36),
	room_id CHAR(36),
    CONSTRAINT PK_Appointment PRIMARY KEY (appointment_id),
    CONSTRAINT FK_Appointment_AppointmentType FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id),
    CONSTRAINT FK_Appointment_Doctor FOREIGN KEY (doctor_id) REFERENCES doctor(doctor_id),
    CONSTRAINT FK_Appointment_Patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT FK_Appointment_Room FOREIGN KEY (room_id) REFERENCES room(room_id)
);