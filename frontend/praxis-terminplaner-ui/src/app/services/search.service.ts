import { Injectable } from '@angular/core';
import { ApiService } from './api.service';
import { SearchDefinitionModel } from '../models/search-definition-model';
import { RoomSearchLayoutComponent } from '../components/search/layouts/room-search-layout/room-search-layout.component';
import { UserSearchLayoutComponent } from '../components/search/layouts/user-search-layout/user-search-layout.component';
import { AppointmentTypeSearchLayoutComponent } from '../components/search/layouts/appointment-type-search-layout/appointment-type-search-layout.component';

@Injectable({
    providedIn: 'root'
})
export class SearchService {

    private static _searchLayouts: Map<String, any> = new Map(Object.entries({
        room: RoomSearchLayoutComponent,
        user: UserSearchLayoutComponent,
        appointmentType: AppointmentTypeSearchLayoutComponent
    }))

    constructor() { }

    async search(searchKey: string, searchClause: string): Promise<any[]> {
        return await ApiService.get("search/" + searchKey, null, { search_clause: searchClause });
    }

    getAvailableSearches(): SearchDefinitionModel[] {
        return [
            {
                searchKey: "user",
                displayName: "Benutzer"
            },
            {
                searchKey: "room",
                displayName: "Raum"
            },
            {
                searchKey: "appointmentType",
                displayName: "Termin-Art"
            }
        ]
    }

    getSearchLayoutComponent(searchKey: string) {
        return SearchService._searchLayouts.get(searchKey);
    }

}
