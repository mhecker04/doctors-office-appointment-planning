import { NgModule } from "@angular/core";
import { RoomSearchLayoutComponent } from "../components/search/layouts/room-search-layout/room-search-layout.component";
import { MaterialModule } from "./material/material.module";
import { UserSearchLayoutComponent } from "../components/search/layouts/user-search-layout/user-search-layout.component";
import { AppointmentTypeSearchLayoutComponent } from "../components/search/layouts/appointment-type-search-layout/appointment-type-search-layout.component";

@NgModule({
    declarations: [
        RoomSearchLayoutComponent,
        UserSearchLayoutComponent,
        AppointmentTypeSearchLayoutComponent
    ],
    imports: [
        MaterialModule
    ],
    exports: [
        RoomSearchLayoutComponent,
        UserSearchLayoutComponent,
        AppointmentTypeSearchLayoutComponent
    ] 
})
export class SearchLayoutModule {

}