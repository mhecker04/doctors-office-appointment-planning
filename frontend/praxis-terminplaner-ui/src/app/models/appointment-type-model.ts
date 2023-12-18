import { BaseModel } from "./base-model";


export class AppointmentTypeModel extends BaseModel<string | undefined>{

    appointment_type_id?: string;
    appointment_type_name: string;
    duration: string;

    constructor(appointmentTypeName: string, durration: string) {
        super();
        this.appointment_type_name = appointmentTypeName;
        this.duration = durration;
    }

    override getPrimaryKey(): string | undefined {
        return this.appointment_type_id;
    }

}