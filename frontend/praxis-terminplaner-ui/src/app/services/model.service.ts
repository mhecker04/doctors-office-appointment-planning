import { Injectable } from '@angular/core';
import { ApiService } from './api.service';

@Injectable({
  providedIn: 'root'
})
export abstract class ModelService<TModel> {

    abstract baseUrl(): string;

    abstract getInitialModel(): TModel

    async create(model: TModel) {
        return ApiService.post(this.baseUrl(), model);
    }

    async get(id: string): Promise<TModel> {
        return ApiService.get(this.baseUrl() + "/" +id, null);
    }

    async delete(id: string) {
        return ApiService.delete(this.baseUrl() + "/" + id, null);
    }

    async update(model: TModel) {
        return ApiService.put(this.baseUrl(), model);
    }

}
