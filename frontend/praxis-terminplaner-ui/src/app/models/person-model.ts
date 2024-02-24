import { BaseModel } from "./base-model";


export class PersonModel extends BaseModel<string> {
    
    person_id?: string;
    firstname: string;
    lastname: string;
    email: string;

    constructor(firstname: string = "", lastname = "", email = "") {
        super();
        this.firstname = firstname;
        this.lastname = lastname;
        this.email = email;
    }

    override getPrimaryKey(): string | undefined {

        return this.person_id

    }

    override setPrimaryKey(primaryKey: string): void {
        this.person_id = primaryKey;
    }

}

