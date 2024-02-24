import { Component } from '@angular/core';
import { ModelComponent } from '../model/model.component';
import { DoctorModel } from 'src/app/models/doctor-model';
import { ActivatedRoute } from '@angular/router';
import { DoctorService } from 'src/app/services/doctor.service';

@Component({
  selector: 'app-doctor',
  templateUrl: `./doctor.component.html`,
  styleUrls: ['./doctor.component.css'],
})
export class DoctorComponent extends ModelComponent<DoctorModel> { 

    doctorService: DoctorService;

    constructor(route: ActivatedRoute, service: DoctorService) {
        super(route, service);
        this.doctorService = service;
    }

}
