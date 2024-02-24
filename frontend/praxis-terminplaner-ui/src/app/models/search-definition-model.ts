import { ModelService } from "../services/model.service";

export interface SearchDefinitionModel {
    searchKey: string,
    displayName: string,
    service: ModelService<any> 
}