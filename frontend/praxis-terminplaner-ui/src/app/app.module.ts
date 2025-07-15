import { NgModule } from "@angular/core";
import { UserComponent } from "./components/user/user.component";
import { LoginComponent } from "./components/login/login.component";
import { HomeComponent } from "./components/home/home.component";
import { ToolbarComponent } from "./components/toolbar/toolbar.component";
import { AppComponent } from "./app.component";
import { CommonModule } from "@angular/common";
import { BrowserModule } from "@angular/platform-browser";
import { MaterialModule } from "./modules/material/material.module";
import { Route, RouterModule } from "@angular/router";
import { AppointmentTypeComponent } from "./components/appointment-type/appointment-type.component";
import { RoomComponent } from "./components/room/room.component";
import { SearchComponent } from "./components/search/search.component";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { SearchLayoutModule } from "./modules/search-layout.module";
import { RoomAppointmentTypeTabComponent } from "./components/room/tabs/room-appointment-type-tab/room-appointment-type-tab.component";
import { SearchDialogComponent } from "./components/dialogs/search-dialog/search-dialog.component";
import { DoctorComponent } from "./components/doctor/doctor.component";
import { DoctorAppointmentTypeTabComponent } from "./components/doctor/tabs/doctor-appointment-type-tab/doctor-appointment-type-tab.component";
import { SearcherComponent } from "./components/search/searcher/searcher.component";
import { AppointmentPlanningComponent } from "./components/appointment-planning/appointment-planning.component";
import { PatientComponent } from './components/patient/patient.component';
import { AppointmentSelectionComponent } from './components/dialogs/appointment-selection/appointment-selection.component';
import { CalendarComponent } from './components/calendar/calendar.component';
import { TimesPipe } from "./pipes/times.pipe";
import { SelectAppointmentResourcesDialogComponent } from './components/dialogs/select-appointment-resources-dialog/select-appointment-resources-dialog.component';

let routes: Route[] = [
    { path: "login", component: LoginComponent },
    { path: "user", component: UserComponent },
    { path: "appointmentType", component: AppointmentTypeComponent },
    { path: "room/:id", component: RoomComponent },
    { path: "room", component: RoomComponent },
    { path: "search", component: SearchComponent },
    { path: "doctor/:id", component: DoctorComponent},
    { path: "doctor", component: DoctorComponent},
    { path: "appointment", component: AppointmentPlanningComponent },
    { path: "patient/:id", component: PatientComponent }
]

@NgModule({
    declarations: [
        RoomComponent,
        AppointmentTypeComponent,
        UserComponent,
        LoginComponent,
        HomeComponent,
        ToolbarComponent,
        AppComponent,
        SearchComponent,
        RoomAppointmentTypeTabComponent,
        SearchDialogComponent,
        DoctorComponent,
        DoctorAppointmentTypeTabComponent,
        SearcherComponent,
        AppointmentPlanningComponent,
        PatientComponent,
        AppointmentSelectionComponent,
        CalendarComponent,
        TimesPipe,
        SelectAppointmentResourcesDialogComponent,
    ],
    imports: [
        BrowserModule,
        MaterialModule,
        SearchLayoutModule,
        CommonModule,
        BrowserAnimationsModule,
        RouterModule.forRoot(routes),
    ],
    exports: [
        HomeComponent,
    ],
    providers: [
    ],
    bootstrap: [AppComponent]
})
export class AppModule {

}
