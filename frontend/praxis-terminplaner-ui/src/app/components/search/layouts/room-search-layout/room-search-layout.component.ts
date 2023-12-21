import { Component, Input } from '@angular/core';
import { RoomModel } from 'src/app/models/room-model';
import { SearchLayoutComponent } from '../search-layout.component';

@Component({
    selector: 'app-room-search-layout',
    templateUrl: './room-search-layout.component.html',
    styleUrls: ['./room-search-layout.component.css'],
})
export class RoomSearchLayoutComponent extends SearchLayoutComponent<RoomModel> {

    override getDisplayColumns(): string[] {
        return ['name', 'number']
    }

}
