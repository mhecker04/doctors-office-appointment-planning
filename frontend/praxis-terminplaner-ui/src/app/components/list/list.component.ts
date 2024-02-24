import { Component, Input } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { BaseModel } from 'src/app/models/base-model';
import { ListService } from 'src/app/services/list.service';
import { SearchDialogComponent } from '../dialogs/search-dialog/search-dialog.component';

@Component({
  selector: 'app-list',
  templateUrl: './list.component.html',
  styleUrls: ['./list.component.css'],
})
export abstract class ListComponent<TParent extends BaseModel<string>, TModel extends BaseModel<string>> { 

    @Input() parent!: TParent;
    @Input() parentTabs!: any[];

    dialog: MatDialog
    models: TModel[];
    listService: ListService<TModel> 

    abstract createNewModelFromSearchModel(model: any): TModel

    constructor(dialog: MatDialog, roomAppointmentTypeService: ListService<TModel>) {
        this.dialog = dialog;
        this.models = [];
        this.listService = roomAppointmentTypeService;
    }

    async ngOnInit(): Promise<void> {

        this.parentTabs.push(this);
        if(this.parent.getPrimaryKey() == null) {
            return;
        }
        this.models = await this.listService.getByParentId(this.parent.getPrimaryKey()!) || [];

    }

    openDialog() {

        let searchDialogReference = this.dialog.open(SearchDialogComponent, {
            data: { searchKey: "appointmentType", onRowClick: this.onRowClick.bind(this) }
        });

    }

    onRowClick(model: any) {
        let newModel = this.createNewModelFromSearchModel(model);

        this.models.push(newModel);
    }

    async save(): Promise<void> {
        await this.listService.saveList(this.models);

    }

}
