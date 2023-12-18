import { Component } from '@angular/core';
import { ModelComponent } from '../model/model.component';
import { AppointmentTypeService } from 'src/app/services/appointment-type.service';
import { AppointmentTypeModel } from 'src/app/models/appointment-type-model';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-appointment-type',
  templateUrl: "./appointment-type.component.html",
  styleUrls: ['./appointment-type.component.css'],
})
export class AppointmentTypeComponent extends ModelComponent<AppointmentTypeModel> { 

    constructor(route: ActivatedRoute, service: AppointmentTypeService) {
        super(route, service);
    }

}
