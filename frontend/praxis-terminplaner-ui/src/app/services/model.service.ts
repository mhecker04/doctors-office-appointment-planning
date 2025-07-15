import { Injectable } from '@angular/core';
import { ApiService } from './api.service';

@Injectable({
  providedIn: 'root'
})
export abstract class ModelService<TModel extends object> {

    abstract baseUrl(): string;

    abstract getInitialModel(): TModel

    async create(model: TModel): Promise<string | null> {
        return ApiService.post(this.baseUrl(), model);
    }

    async get(id: string): Promise<TModel> {
        let apiModel = await ApiService.get(this.baseUrl() + "/" +id, null);

        let model = this.getInitialModel();
        Object.assign(model, apiModel);

        return model;

    }

    async delete(id: string) {
        return ApiService.delete(this.baseUrl() + "/" + id, null);
    }

    async update(model: TModel) {
        return ApiService.put(this.baseUrl(), model);
    }

    async getAll(): Promise<TModel[] | null> {
        return await ApiService.get("search/" + this.baseUrl(), null, {search_clause: ""});
    }

}
