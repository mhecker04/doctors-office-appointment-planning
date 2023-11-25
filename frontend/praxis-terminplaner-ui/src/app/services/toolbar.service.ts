import { Injectable } from '@angular/core';
import { ToolbarButton } from '../models/toolbar-button';

@Injectable({
  providedIn: 'root'
})
export class ToolbarService {

  constructor() { }

  GetToolbarButtons(): ToolbarButton[] {
    return [
      {Icon: 'home'},
      {Icon: 'settings'}

    ]
  }

}
