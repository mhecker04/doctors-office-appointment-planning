import { Component } from '@angular/core';
import { SearchDefinitionModel } from 'src/app/models/search-definition-model';
import { SearchService } from 'src/app/services/search.service';

@Component({
    selector: 'app-search',
    templateUrl: './search.component.html',
    styleUrls: ['./search.component.css'],
})
export class SearchComponent {

    searchService: SearchService;
    searchKey: string;
    searchClause: string;
    searchDefinitions: SearchDefinitionModel[];
    models: any[];

    constructor(searchService: SearchService) {
        this.searchService = searchService;
        this.searchKey = "";
        this.searchClause = "";
        this.searchDefinitions = this.searchService.getAvailableSearches();
        this.models = [];
    }

    async search() {

        let searchModels = await this.searchService.search(this.searchKey, this.searchClause);

        if (searchModels == null) {
            console.error("Search failed");
            return;
        }

        this.models = searchModels;
    }

}
