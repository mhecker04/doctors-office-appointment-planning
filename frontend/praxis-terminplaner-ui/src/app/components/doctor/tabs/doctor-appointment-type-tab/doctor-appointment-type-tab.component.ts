import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { ListComponent } from 'src/app/components/list/list.component';
import { DoctorAppointmentTypeModel } from 'src/app/models/doctor-appointment-type-model';
import { DoctorModel } from 'src/app/models/doctor-model';
import { DoctorAppointmentTypeService } from 'src/app/services/doctor-appointment-type.service';

@Component({
  selector: 'app-doctor-appointment-type-tab',
  templateUrl: './doctor-appointment-type-tab.component.html',
  styleUrls: ['./doctor-appointment-type-tab.component.css'],
})
export class DoctorAppointmentTypeTabComponent extends ListComponent<DoctorModel, DoctorAppointmentTypeModel>{
    
    constructor(dialog: MatDialog, doctorAppointmentTypeService: DoctorAppointmentTypeService) {
        super(dialog, doctorAppointmentTypeService);
    }

    override createNewModelFromSearchModel(model: any): DoctorAppointmentTypeModel {

        let newModel = new DoctorAppointmentTypeModel();
        newModel.doctor_id = this.parent.doctor_id!;
        newModel.appointment_type_id = model.appointment_type_id;
        newModel.appointment_type = model;
        return newModel;

    }
}
