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

    ApiService.Login(this.username, this.password)
      .then(success => {
        
        if (!success) {

        console.log(success);
          this.loginFailed = true;
          this.test = "nachher";
          console.log(this);
          
        }
      });

  }

}
