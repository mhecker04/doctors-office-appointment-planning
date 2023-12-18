import { Component } from '@angular/core';
import { UserModel } from 'src/app/models/user-model';
import { UserService } from 'src/app/services/user.service';
import { ModelComponent } from '../model/model.component';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-user',
  templateUrl: `./user.component.html`,
  styleUrls: ['./user.component.css'],
})
export class UserComponent extends ModelComponent<UserModel>{ 

  constructor(route: ActivatedRoute, userService: UserService) {
    super(route, userService);
  }

}
