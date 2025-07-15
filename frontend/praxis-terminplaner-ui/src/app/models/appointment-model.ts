import { BaseModel } from "./base-model";


export class AppointmentModel extends BaseModel<string> {

    appointment_id?: string;
    appointment_type_id: string;
    doctor_id: string;
    start_date_time: Date;
    end_date_time: Date;
    patient_id: string;
    room_id: string;


    constructor(appointmentTypeId: string,
                doctorId: string, from: Date, to: Date, patientId: string, roomId: string) {
        super();
        this.appointment_type_id = appointmentTypeId;
        this.doctor_id = doctorId;
        this.start_date_time = from;
        this.end_date_time = to;
        this.patient_id = patientId;
        this.room_id = roomId;
    }

    override getPrimaryKey(): string | undefined {
        return this.appointment_id;
    }

    override setPrimaryKey(primaryKey: string): void {
        this.appointment_id = primaryKey;
    }

}

