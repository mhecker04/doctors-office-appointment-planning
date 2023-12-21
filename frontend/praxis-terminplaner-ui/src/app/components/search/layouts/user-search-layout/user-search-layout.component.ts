import { Component } from '@angular/core';
import { SearchLayoutComponent } from '../search-layout.component';
import { UserModel } from 'src/app/models/user-model';

@Component({
  selector: 'app-user-search-layout',
  templateUrl: './user-search-layout.component.html',
  styleUrls: ['./user-search-layout.component.css'],
})
export class UserSearchLayoutComponent extends SearchLayoutComponent<UserModel> {
    
    override getDisplayColumns(): string[] {

        return ["username"]

    }

}
