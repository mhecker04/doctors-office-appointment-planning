import { Component } from '@angular/core';
import { FormControl } from '@angular/forms';
import { AvailableRessourcesModel } from 'src/app/models/available-appointment-ressources-model';
import { AppointmentService } from 'src/app/services/appointment.service';


@Component({
    selector: 'app-appointment-planning',
    templateUrl: './appointment-planning.component.html',
    styleUrls: ['./appointment-planning.component.css'],

})
export class AppointmentPlanningComponent {

    appointmentTypeId: string;
    datetime: Date;
    appointmentService: AppointmentService;
    availableRessources: AvailableRessourcesModel;

    constructor(appointmentService: AppointmentService) {
        this.appointmentTypeId = "";
        this.datetime = new Date();
        this.appointmentService = appointmentService;
        this.availableRessources = new AvailableRessourcesModel();
    }

    setAppointmentTypeId(appointmentTypeId: string) {
        this.appointmentTypeId = appointmentTypeId;
    }

    async getAvailableRessources() {
        this.availableRessources =
            await this.appointmentService.getAvailableRessources(this.appointmentTypeId, this.datetime) || this.availableRessources;
        
    }

}
