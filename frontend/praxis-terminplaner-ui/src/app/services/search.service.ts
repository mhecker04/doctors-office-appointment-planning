import { Injectable } from '@angular/core';
import { ApiService } from './api.service';
import { SearchDefinitionModel } from '../models/search-definition-model';
import { RoomSearchLayoutComponent } from '../components/search/layouts/room-search-layout/room-search-layout.component';
import { UserSearchLayoutComponent } from '../components/search/layouts/user-search-layout/user-search-layout.component';
import { AppointmentTypeSearchLayoutComponent } from '../components/search/layouts/appointment-type-search-layout/appointment-type-search-layout.component';
import { BaseModel } from '../models/base-model';
import { UserService } from './user.service';
import { RoomService } from './room.service';
import { AppointmentTypeService } from './appointment-type.service';

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

    async search(searchKey: string, searchClause: string): Promise<any[] | null> {
        let models: BaseModel<string>[] | null = await ApiService.get("search/" + searchKey, null, { search_clause: searchClause });
        if (models == null) {
            return models;
        }
        return this.mapToModels(searchKey, models!);
    }

    getAvailableSearches(): SearchDefinitionModel[] {
        return [
            {
                searchKey: "user",
                displayName: "Benutzer",
                service: new UserService()
            },
            {
                searchKey: "room",
                displayName: "Raum",
                service: new RoomService()
            },
            {
                searchKey: "appointmentType",
                displayName: "Termin-Art",
                service: new AppointmentTypeService()
            }
        ]
    }

    getSearchLayoutComponent(searchKey: string) {
        return SearchService._searchLayouts.get(searchKey);
    }

    mapToModels(searchKey: string, models: BaseModel<string>[]): BaseModel<string>[] {

        let service = this.getAvailableSearches().find(searchDefinition => searchDefinition.searchKey == searchKey)?.service;

        return models.map(m => {
            let newModel = service?.getInitialModel();

            Object.assign(newModel, m);
            return newModel;

        })

    }

}
