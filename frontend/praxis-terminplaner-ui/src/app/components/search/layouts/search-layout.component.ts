import { Component, Input } from "@angular/core";


@Component ({
    template: '' 
})
export abstract class SearchLayoutComponent<TModel> {

    @Input() models!: TModel[];
    @Input() onRowClick!: (model: TModel)=> void;

    constructor() {

    }

    abstract getDisplayColumns(): string[];

}