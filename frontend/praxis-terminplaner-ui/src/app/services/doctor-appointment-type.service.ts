import { Injectable } from '@angular/core';
import { ListService } from './list.service';
import { DoctorAppointmentTypeModel } from '../models/doctor-appointment-type-model';

@Injectable({
    providedIn: 'root'
})
export class DoctorAppointmentTypeService extends ListService<DoctorAppointmentTypeModel>{

    override getInitialModel(): DoctorAppointmentTypeModel {
        return new DoctorAppointmentTypeModel();
    }

    override baseUrl(): string {
        return "doctor";
    }

    override getParentIdFromModel(model: DoctorAppointmentTypeModel): string {
        return model.doctor_id;
    }

    override childUrl(): string {
        return "appointmentTypes";
    }

}
