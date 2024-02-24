import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';

import { OnInit } from '@angular/core';
import { ModelService } from 'src/app/services/model.service';
import { ActivatedRoute } from '@angular/router';
import { throwToolbarMixedModesError } from '@angular/material/toolbar';
import { BaseModel } from 'src/app/models/base-model';

@Component({
    template: ``,
})
export class ModelComponent<TModel extends BaseModel<string>> implements OnInit {

    model!: TModel;
    modelService: ModelService<TModel>
    route: ActivatedRoute;
    tabs: any[];

    constructor(route: ActivatedRoute, modelService: ModelService<TModel>) {
        this.modelService = modelService;
        this.route = route;
        this.tabs = [];
    }

    async ngOnInit(): Promise<void> {
        let id = this.getIdFromRoute();
        if (id) {
            this.model = await this.modelService.get(id!);
            return;
        }
        this.model = this.modelService.getInitialModel();

    }

    async saveTabs() {

        let promises = [];

        console.log(this.tabs);
        

        for (let tab of this.tabs) {
            promises.push(tab.save());
        }
        return Promise.all(promises);
    }

    async create() {
        let primaryKey = await this.modelService.create(this.model)
        if(primaryKey == null) {
            console.error("failed to create entity");
            return;
        }
        this.model.setPrimaryKey(primaryKey);
        await this.saveTabs();

    }

    async update() {
        await this.modelService.update(this.model)
        await this.saveTabs();
    }

    async save() {
        if (this.model.getPrimaryKey() == null) {
            return await this.create();
        }
        return await this.update();
    }

    private getIdFromRoute() {
        return this.route.snapshot.paramMap.get("id");
    }

}
