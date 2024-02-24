import { Injectable } from '@angular/core';
import { ModelService } from './model.service';
import { DoctorModel } from '../models/doctor-model';

@Injectable({
    providedIn: 'root'
})
export class DoctorService extends ModelService<DoctorModel>{

    override baseUrl(): string {
        return "doctor";
    }
    override getInitialModel(): DoctorModel {
        return new DoctorModel();
    }

    constructor() { 
        super();
    }

}
