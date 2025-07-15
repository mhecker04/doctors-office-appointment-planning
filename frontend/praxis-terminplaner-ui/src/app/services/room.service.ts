import { Injectable } from '@angular/core';
import { ModelService } from './model.service';
import { RoomModel } from '../models/room-model';

@Injectable({
  providedIn: 'root'
})
export class RoomService extends ModelService<RoomModel> {

  override baseUrl(): string {
    return "room";
  }
  override getInitialModel(): RoomModel {
    let model = new RoomModel("", "");
    return model;
  }

  constructor() {
    super();
  }

}
