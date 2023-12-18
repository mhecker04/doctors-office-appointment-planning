import { Injectable } from '@angular/core';
import { UserModel } from '../models/user-model';
import { ModelService } from './model.service';

@Injectable({
    providedIn: 'root'
})
export class UserService extends ModelService<UserModel> {
    
    override getInitialModel(): UserModel {

        return new UserModel("", "");

    }

    override baseUrl(): string {
        return "user";
    }

}
