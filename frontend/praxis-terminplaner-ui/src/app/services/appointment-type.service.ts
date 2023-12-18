import { Injectable } from '@angular/core';
import { ModelService } from './model.service';
import { AppointmentTypeModel } from '../models/appointment-type-model';

@Injectable({
    providedIn: 'root'
})
export class AppointmentTypeService extends ModelService<AppointmentTypeModel> {

    override baseUrl(): string {
        return "appointmentType";
    }

    override getInitialModel(): AppointmentTypeModel {
        let model = new AppointmentTypeModel("", "0:00");
        model.appointment_type_id = "";
        return model;
    }

}
