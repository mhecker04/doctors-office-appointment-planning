import { BaseModel } from "./base-model";


export class AppointmentModel extends BaseModel<string> {
    
    appointment_id?: string;
    appointment_type_id: string;
    doctor_id: string;
    from: Date;
    patient_id: string;
    room_id: string;  

    constructor() {
        super();
        this.appointment_type_id = "";
        this.doctor_id = "";
        this.from = new Date();
        this.patient_id = "";
        this.room_id = "";
    }

    override getPrimaryKey(): string | undefined {
        return this.appointment_id;
    }

    override setPrimaryKey(primaryKey: string): void {
        this.appointment_id = primaryKey;
    }

}

