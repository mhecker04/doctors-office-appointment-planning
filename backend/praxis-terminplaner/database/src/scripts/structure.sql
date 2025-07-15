USE appointment_planning;

CREATE TABLE IF NOT EXISTS role 
( 
    role_id CHAR(36), 
    role_name VARCHAR(64), 
    CONSTRAINT pk_role PRIMARY KEY (role_id) 
); 

CREATE TABLE IF NOT EXISTS permission 
( 
    permission_id CHAR(36), 
    permission_name VARCHAR(64), 
    web_url VARCHAR(128), 
    parent_permission_id CHAR(36), 
    CONSTRAINT pk_permission PRIMARY KEY (permission_id),
    CONSTRAINT fk_permission_parent_permission FOREIGN KEY (parent_permission_id) REFERENCES permission(permission_id) 
); 

CREATE TABLE IF NOT EXISTS user (
    user_id CHAR(36),
    username VARCHAR(64),
    password VARCHAR(60),
    CONSTRAINT pk_user PRIMARY KEY (user_id)
);

CREATE TABLE IF NOT EXISTS user_permission
(
    user_permission_id CHAR(36),
    user_id CHAR(36),
    permission_id CHAR(36),
    CONSTRAINT pk_user_permission PRIMARY KEY (user_permission_id),
    CONSTRAINT fk_user_permission_user FOREIGN KEY (user_id) REFERENCES user(user_id),
    CONSTRAINT fk_user_permission_permission FOREIGN KEY (permission_id) REFERENCES permission(permission_id)
);

CREATE TABLE IF NOT EXISTS role_permission 
( 
    role_permission_id CHAR(36), 
    role_id CHAR(36), 
    permission_id CHAR(36), 
    CONSTRAINT pk_role_permission PRIMARY KEY (role_permission_id), 
    CONSTRAINT fk_role_permission_role FOREIGN KEY (role_id) REFERENCES role(role_id), 
    CONSTRAINT fk_role_permission_permission FOREIGN KEY (permission_id) REFERENCES permission(permission_id) 
); 

CREATE TABLE IF NOT EXISTS appointment_type
(
    appointment_type_id CHAR(36),
    appointment_type_name VARCHAR(64),
    duration TIME,
    CONSTRAINT pk_appointment_type PRIMARY KEY (appointment_type_id)
);  

CREATE TABLE IF NOT EXISTS room (
    room_id CHAR(36),
    room_name VARCHAR(32),
    room_number DECIMAL(5,0),
    CONSTRAINT pk_room PRIMARY KEY (room_id)
);

CREATE TABLE IF NOT EXISTS room_appointment_type
(
    room_appointment_type_id CHAR(36),
    room_id CHAR(36),
    appointment_type_id CHAR(36),
    CONSTRAINT pk_room_appointment_type PRIMARY KEY (room_appointment_type_id),
    CONSTRAINT fk_room_appointment_type_room FOREIGN KEY (room_id) REFERENCES room(room_id),
    CONSTRAINT fk_room_appointment_type_appointment_type FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id)
);

CREATE TABLE IF NOT EXISTS person (
    person_id CHAR(36) NOT NULL,
    lastname VARCHAR(64) NOT NULL,
    firstname VARCHAR(64) NOT NULL,
    email VARCHAR(255),
    user_id CHAR(36),
    CONSTRAINT pk_person PRIMARY KEY (person_id),
    CONSTRAINT fk_person_user FOREIGN KEY (user_id) REFERENCES user(user_id)
);

CREATE TABLE IF NOT EXISTS doctor
(
    doctor_id CHAR(36),
    person_id CHAR(36),
    CONSTRAINT pk_doctor PRIMARY KEY (doctor_id),
    CONSTRAINT fk_doctor_user FOREIGN KEY (person_id) REFERENCES person(person_id)
);

CREATE TABLE IF NOT EXISTS doctor_appointment_type
(
    doctor_appointment_type_id CHAR(36),
    appointment_type_id CHAR(36),
    doctor_id CHAR(36),
    CONSTRAINT pk_doctor_appointment_type PRIMARY KEY (doctor_appointment_type_id),
    CONSTRAINT fk_doctor_appointment_type_appointment_type FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id),
    CONSTRAINT fk_doctor_appointment_type_doctor FOREIGN KEY (doctor_id) REFERENCES doctor(doctor_id)
);

CREATE TABLE IF NOT EXISTS patient
(
    patient_id CHAR(36),
    person_id CHAR(36),
    CONSTRAINT pk_patient PRIMARY KEY (patient_id),
    CONSTRAINT fk_patient_person FOREIGN KEY (person_id) REFERENCES person(person_id)
);

CREATE TABLE IF NOT EXISTS appointment
(
    appointment_id CHAR(36),
    appointment_type_id CHAR(36),
    doctor_id CHAR(36),
    start_date_time DATETIME,
    end_date_time DATETIME,
    patient_id CHAR(36),
    room_id CHAR(36),
    CONSTRAINT pk_appointment PRIMARY KEY (appointment_id),
    CONSTRAINT fk_appointment_appointment_type FOREIGN KEY (appointment_type_id) REFERENCES appointment_type(appointment_type_id),
    CONSTRAINT fk_appointment_doctor FOREIGN KEY (doctor_id) REFERENCES doctor(doctor_id),
    CONSTRAINT fk_appointment_patient FOREIGN KEY (patient_id) REFERENCES patient(patient_id),
    CONSTRAINT fk_appointment_room FOREIGN KEY (room_id) REFERENCES room(room_id)
);
