import { AppointmentTypeModel } from "./appointment-type-model";
import { BaseModel } from "./base-model";

export class RoomAppointmentTypeModel extends BaseModel<string>{
    

    room_appointment_type_id: string | undefined;
    appointment_type_id: string;
    room_id: string;
    appointment_type: AppointmentTypeModel | undefined;

    constructor() {
        super();
        this.appointment_type_id = "";
        this.room_id = "";
    }

    override getPrimaryKey(): string | undefined {
        return this.room_appointment_type_id;
    }

    override setPrimaryKey(primaryKey: string | undefined): void {
        this.room_appointment_type_id = primaryKey;
    }

}
