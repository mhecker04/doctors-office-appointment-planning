import { Injectable } from '@angular/core';
import { BaseModel } from '../models/base-model';
import { ApiService } from './api.service';

@Injectable({
    providedIn: 'root'
})
export abstract class ListService<TModel extends BaseModel<string>> {


    abstract baseUrl(): string;
    abstract getParentIdFromModel(model: TModel): string;
    abstract childUrl(): string;


    constructor() { }

    async getByParentId(parentId: string): Promise<TModel[] | null> {
        return ApiService.get(`${this.baseUrl()}/${parentId}/${this.childUrl()}`, null);
    }

    async save(model: TModel): Promise<string | null> {
        return ApiService.post(`${this.baseUrl()}/${this.getParentIdFromModel(model)}/${this.childUrl()}`, model);
    }

    async saveList(models: TModel[]): Promise<(string | null)[]> {

        let promises = [];

        for (let model of models) {
            let primaryKey = model.getPrimaryKey();
            if (primaryKey == null || primaryKey == "") {
                promises.push(this.save(model))
            }
        }

        return Promise.all(promises);

    }

}
