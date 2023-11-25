import { Component } from '@angular/core';
import { ToolbarButton } from 'src/app/models/toolbar-button';
import { ToolbarService } from 'src/app/services/toolbar.service';

@Component({
  selector: 'app-toolbar',
  templateUrl: './toolbar.component.html',
  styleUrls: ['./toolbar.component.css'],

})
export class ToolbarComponent {

  ToolbarButtons: ToolbarButton[] = []

  ToolbarService: ToolbarService
  
  constructor(toolbarService: ToolbarService) {
    this.ToolbarService = toolbarService; 
  }
  
}
