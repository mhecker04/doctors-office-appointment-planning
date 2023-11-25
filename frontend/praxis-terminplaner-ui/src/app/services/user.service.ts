import { Injectable } from '@angular/core';
import { UserModel } from '../models/user';
import { ApiService } from './api.service';

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor() { }

  save(user: UserModel) {

    let result = ApiService.Post("http://127.0.0.1:8000/user", user);
    
  }

}
