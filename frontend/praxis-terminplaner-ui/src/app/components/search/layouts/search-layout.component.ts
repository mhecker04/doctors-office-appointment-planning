import { Component, Input } from "@angular/core";


@Component ({
    template: '' 
})
export abstract class SearchLayoutComponent<TModel> {

    @Input() models!: TModel[];

    constructor() {

    }

    abstract getDisplayColumns(): string[];

}