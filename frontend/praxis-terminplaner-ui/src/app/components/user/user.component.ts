import { Component } from '@angular/core';
import { UserModel } from 'src/app/models/user';
import { UserService } from 'src/app/services/user.service';



@Component({
  selector: 'app-user',
  templateUrl: `./user.component.html`,
  styleUrls: ['./user.component.css'],
})
export class UserComponent { 

  Username = "";
  Password =  "";
  UserId: string;
  UserService: UserService;

  constructor(userService: UserService) {

    this.UserService = userService;

    this.UserId = "";
  }

  Save() {

    let user: UserModel = {
      username:  this.Username,
      password: this.Password,
      user_id: this.UserId
    }

    this.UserService.save(user)

  }

}
