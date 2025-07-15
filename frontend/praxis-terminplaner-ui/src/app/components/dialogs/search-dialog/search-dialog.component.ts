import { Component, Inject } from '@angular/core';
import { MAT_DIALOG_DATA } from '@angular/material/dialog';
import { SearchService } from 'src/app/services/search.service';

export interface SearchDialogData {
    searchKey: string,
    onRowClick: (model: any) => void
}

@Component({
  selector: 'app-search-dialog',
  templateUrl: './search-dialog.component.html',
  styleUrls: ['./search-dialog.component.css'],
})
export class SearchDialogComponent {

    searchService: SearchService;
    searchClause: string;
    models: any[];

    constructor(searchService: SearchService, @Inject(MAT_DIALOG_DATA) public data: SearchDialogData) {
        this.searchService = searchService;
        this.searchClause = "";
        this.models = [];
    }

    async search() {

        this.models = await this.searchService.search(this.data.searchKey, this.searchClause) || [];
    }

}
