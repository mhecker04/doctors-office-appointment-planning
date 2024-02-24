import { BaseModel } from "./base-model";
import { PersonModel } from "./person-model";

export class DoctorModel extends BaseModel<string> {
    
    doctor_id?: string;    
    person_id?: string;
    person: PersonModel;

    constructor() {
        super();
        this.person = new PersonModel();
    }

    override getPrimaryKey(): string | undefined {
        return this.doctor_id;
    }
    override setPrimaryKey(primaryKey: string): void {
        this.doctor_id = primaryKey;
    }

}
