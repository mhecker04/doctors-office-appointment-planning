import { Injectable } from '@angular/core';
import { RoomAppointmentTypeModel } from '../models/room-appointment-type-model';
import { ApiService } from './api.service';
import { ListService } from './list.service';

@Injectable({
    providedIn: 'root'
})
export class RoomAppointmentTypeService extends ListService<RoomAppointmentTypeModel> {

    override baseUrl(): string {
        return "room";
    }
    override getParentIdFromModel(model: RoomAppointmentTypeModel): string {
        return model.room_id;
    }
    override childUrl(): string {
        return "appointmentTypes";
    }

}
