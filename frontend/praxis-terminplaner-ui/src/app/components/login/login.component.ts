import { Component } from '@angular/core';
import { ApiService } from 'src/app/services/api.service';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css'],
})
export class LoginComponent {

  loginFailed: any;
  username: string;
  password: string;
  test: string = "vorher"

  constructor() {
    this.username = "";
    this.password = "";
    this.loginFailed = false;
  }

  login() {

    ApiService.login(this.username, this.password)
      .then(success => {
        
        if (!success) {

          this.loginFailed = true;
          this.test = "nachher";
          
        }
      });

  }

}
