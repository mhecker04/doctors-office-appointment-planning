import { Injectable } from '@angular/core';
import { ModelService } from './model.service';
import { AppointmentModel } from '../models/appointment-model';
import { ApiService } from './api.service';
import { AvailableRessourcesModel } from '../models/available-appointment-ressources-model';

@Injectable({
    providedIn: 'root'
})
export class AppointmentService extends ModelService<AppointmentModel> {

    constructor() {
        super();
    }

    override baseUrl(): string {
        return "appointment";
    }

    override getInitialModel(): AppointmentModel {
        return new AppointmentModel();
    }

    getAvailableRessources(appointmentTypeId: string, datetime: Date): Promise<AvailableRessourcesModel | null>  {

        let queryParameters = {
            appointment_type_id: appointmentTypeId, 
            datetime: datetime.toJSON()
        }

        return ApiService.get(
            this.baseUrl() + "/" + "resources",
            null,
            queryParameters
        )

    }

}
