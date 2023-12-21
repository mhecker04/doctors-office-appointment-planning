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
import { MatInputModule } from "@angular/material/input";
import { MatFormFieldModule } from "@angular/material/form-field";
import { SearchLayoutModule } from "./modules/search-layout.module";
import { RoomSearchLayoutComponent } from "./components/search/layouts/room-search-layout/room-search-layout.component";



let routes: Route[] = [
    { path: "login", component: LoginComponent },
    { path: "user", component: UserComponent },
    { path: "", component: AppComponent },
    { path: "appointmentType", component: AppointmentTypeComponent },
    { path: "room", component: RoomComponent },
    { path: "search", component: SearchComponent },
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