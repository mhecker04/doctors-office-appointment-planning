import { BaseModel } from "./base-model";


export class RoomModel extends BaseModel<string | undefined> {

    room_id?: string;
    room_name: string;
    room_number: string;

    constructor(roomName: string, roomNumber: string) {
        super();
        this.room_name = roomName;
        this.room_number = roomNumber;
    }

    override getPrimaryKey(): string | undefined {
        return this.room_id;
    }

    override setPrimaryKey(primaryKey: string | undefined): void {
        this.room_id = primaryKey;
    }

}
