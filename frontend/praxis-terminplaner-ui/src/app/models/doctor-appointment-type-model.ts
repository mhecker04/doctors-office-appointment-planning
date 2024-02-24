import { BaseModel } from "./base-model";
import { DoctorModel } from "./doctor-model";


export class DoctorAppointmentTypeModel extends BaseModel<string>{
    
    doctor_appointment_type_id: string | undefined;
    appointment_type_id: string;
    doctor_id: string;
    appointment_type: DoctorModel | undefined;

    constructor() {
        super();
        this.appointment_type_id = "";
        this.doctor_id = "";
    }

    override getPrimaryKey(): string | undefined {
        return this.doctor_appointment_type_id;
    }

    override setPrimaryKey(primaryKey: string | undefined): void {
        this.doctor_appointment_type_id = primaryKey;
    }

}