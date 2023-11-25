import { Token } from '../models/token';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class ApiService {
  
  private static BASE_URL = "http://localhost:8000/";


  static async Login(username: string, password: string): Promise<boolean> {

    let requestInit: RequestInit = {
      method: 'POST',
      body: JSON.stringify({
        username, password
      })
    }

    let success = true;

    let result;

    try {
      result = await fetch(this.BASE_URL + "auth/login", requestInit);
    } catch {
      return false;
    }

      

    if(!success) {
      return false;
    }

    let token: Token = await result.json();

    localStorage.setItem("token", token.access_token);

    return true;

  }

  static async Post<TParameters, TResponse>(url: string, parameters: TParameters): Promise<TResponse> {

    let token = localStorage.getItem("token");

    let headers: HeadersInit = {
      "Authorization": "Bearer " + token
    }

    let requestInit: RequestInit = {
      method: 'POST',
      headers: headers,
      body: JSON.stringify(parameters)
    } 

    let result = await fetch(url, requestInit);

    return await result.json() 

  }

}
