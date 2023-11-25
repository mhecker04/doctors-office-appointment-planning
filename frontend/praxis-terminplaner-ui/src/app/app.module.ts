import { NgModule } from "@angular/core";
import { UserComponent } from "./components/user/user.component";
import { LoginComponent } from "./components/login/login.component";
import { HomeComponent } from "./components/home/home.component";
import { ToolbarComponent } from "./components/toolbar/toolbar.component";
import { AppComponent } from "./app.component";
import { CommonModule } from "@angular/common";
import { BrowserModule } from "@angular/platform-browser";
import { MaterialModule } from "./modules/material/material.module";
import { Route, RouterModule } from "@angular/router";




let routes: Route[] = [
    { path: "login", component: LoginComponent },
    { path: "user", component: UserComponent },
    { path: "", component: AppComponent}
] 


@NgModule({
    declarations: [
        UserComponent,
        LoginComponent,
        HomeComponent,
        ToolbarComponent,
        AppComponent,
    ],
    imports: [
        BrowserModule,
        MaterialModule,
        CommonModule,
        RouterModule.forRoot(routes)
    ],
    exports: [
        HomeComponent,
    ],
    providers: [
    ],
    bootstrap: [AppComponent]
})
export class AppModule {

}