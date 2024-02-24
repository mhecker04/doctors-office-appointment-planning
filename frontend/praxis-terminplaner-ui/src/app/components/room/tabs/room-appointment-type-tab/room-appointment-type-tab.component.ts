import { Component, Input, OnInit } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { SearchDialogComponent } from 'src/app/components/dialogs/search-dialog/search-dialog.component';
import { ListComponent } from 'src/app/components/list/list.component';
import { AppointmentTypeModel } from 'src/app/models/appointment-type-model';
import { RoomAppointmentTypeModel } from 'src/app/models/room-appointment-type-model';
import { RoomModel } from 'src/app/models/room-model';
import { RoomAppointmentTypeService } from 'src/app/services/room-appointment-type.service';

@Component({
    selector: 'app-room-appointment-type-tab',
    templateUrl: './room-appointment-type-tab.component.html',
    styleUrls: ['./room-appointment-type-tab.component.css'],
})
export class RoomAppointmentTypeTabComponent extends ListComponent<RoomModel, RoomAppointmentTypeModel> implements OnInit {
    
    roomAppointmentTypeService: RoomAppointmentTypeService

    constructor(dialog: MatDialog, roomAppointmentTypeService: RoomAppointmentTypeService) {
        super(dialog, roomAppointmentTypeService);
        this.dialog = dialog;
        this.models = [];
        this.roomAppointmentTypeService = roomAppointmentTypeService;
    }

    override createNewModelFromSearchModel(model: any): RoomAppointmentTypeModel {
        let newModel = new RoomAppointmentTypeModel()
        newModel.appointment_type_id = model.appointment_type_id!;
        newModel.room_id = this.parent.room_id!;
        newModel.appointment_type = model;
        return newModel;
    }

}
