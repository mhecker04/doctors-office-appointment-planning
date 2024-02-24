import { Component } from '@angular/core';
import { ModelComponent } from '../model/model.component';
import { RoomModel } from 'src/app/models/room-model';
import { ActivatedRoute } from '@angular/router';
import { RoomService } from 'src/app/services/room.service';

@Component({
    selector: 'app-room',
    templateUrl: './room.component.html',
    styleUrls: ['./room.component.css'],
})
export class RoomComponent extends ModelComponent<RoomModel> {

    constructor(route: ActivatedRoute, roomService: RoomService) {
        super(route, roomService);
    }

}
