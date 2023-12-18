import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, Component } from '@angular/core';

import { OnInit } from '@angular/core';
import { ModelService } from 'src/app/services/model.service';
import { ActivatedRoute } from '@angular/router';
import { throwToolbarMixedModesError } from '@angular/material/toolbar';

@Component({
    template: ``,
})
export class ModelComponent<TModel> implements OnInit {

    model!: TModel;
    modelService: ModelService<TModel>
    route: ActivatedRoute;

    constructor(route: ActivatedRoute, modelService: ModelService<TModel>) {
        this.modelService = modelService;
        this.route = route;
    }

    async ngOnInit(): Promise<void> {
        let id = this.getIdFromRoute();
        if (id) {
            this.model = await this.modelService.get(id!);
            return;
        }
        this.model = this.modelService.getInitialModel();
        console.log(this.model);
        
    }

    async create() {
        await this.modelService.create(this.model);
    }

    async update() {
        await this.modelService.update(this.model)
    }

    private getIdFromRoute() {
        return this.route.snapshot.paramMap.get("id");
    }

}
