import { Component } from '@angular/core';
import { FormControl } from '@angular/forms';


@Component({
  selector: 'app-appointment-planning',
  templateUrl: './appointment-planning.component.html',
  styleUrls: ['./appointment-planning.component.css'],

})
export class AppointmentPlanningComponent {

  AvailableWeekdays: string[] = ['Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag']

  Weekdays = new FormControl();

  From = new FormControl();

  To = new FormControl();



}
