import { BaseModel } from "./base-model";

export class UserModel extends BaseModel<string | undefined> {
    username: string;
    user_id?: string;
    password: string;

    constructor(username: string, password: string) { 
        super();
        this.username = username;
        this.password = password;
    }

    override getPrimaryKey(): string | undefined {
        return this.user_id;
    }

    override setPrimaryKey(primaryKey: string | undefined): void {
        this.user_id = primaryKey;
    }

}