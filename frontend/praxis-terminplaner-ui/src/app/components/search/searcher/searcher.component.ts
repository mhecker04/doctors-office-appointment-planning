import { ChangeDetectionStrategy, Component, EventEmitter, Input, Output } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { SearchDialogComponent } from '../../dialogs/search-dialog/search-dialog.component';

@Component({
    selector: 'app-searcher',
    templateUrl: './searcher.component.html',
    styleUrls: ['./searcher.component.css'],
})
export class SearcherComponent {

    dialog: MatDialog;

    @Input() searchKey!: string;
    @Input() objectValue!: string;
    @Output() objectId = new EventEmitter<string>();
    
    objectText: string;

    constructor(dialog: MatDialog) {
        this.dialog = dialog;
        this.objectText = "";
    }

    openDialog() {
        this.dialog.open(SearchDialogComponent, {
            data: { searchKey: this.searchKey, onRowClick: this.onRowClick.bind(this) }
        })
    }

    onRowClick(model: any) {

        this.objectId.emit(model.getPrimaryKey());
        this.objectText = model[this.objectValue];

    }

}
