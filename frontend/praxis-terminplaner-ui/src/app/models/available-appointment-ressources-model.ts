import { DoctorModel } from "./doctor-model";
import { RoomModel } from "./room-model";


export class AvailableRessourcesModel {

    doctors: DoctorModel[];
    rooms: RoomModel[];   

    constructor() {
        this.doctors = [];
        this.rooms = [];
    }

}