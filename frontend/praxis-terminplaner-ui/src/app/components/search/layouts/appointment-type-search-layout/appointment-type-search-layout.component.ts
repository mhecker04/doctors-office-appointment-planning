import { Component } from '@angular/core';
import { SearchLayoutComponent } from '../search-layout.component';
import { AppointmentTypeModel } from 'src/app/models/appointment-type-model';

@Component({
  selector: 'app-appointment-type',
  templateUrl: './appointment-type-search-layout.component.html',
  styleUrls: ['./appointment-type-search-layout.component.css'],
})
export class AppointmentTypeSearchLayoutComponent extends SearchLayoutComponent<AppointmentTypeModel> {
    override getDisplayColumns(): string[] {

        return ["appointmentTypeName", "duration"];

    }
}
