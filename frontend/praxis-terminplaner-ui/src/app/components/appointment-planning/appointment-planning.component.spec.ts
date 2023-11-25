import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AppointmentPlanningComponent } from './appointment-planning.component';

describe('AppointmentPlanningComponent', () => {
  let component: AppointmentPlanningComponent;
  let fixture: ComponentFixture<AppointmentPlanningComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [AppointmentPlanningComponent]
    });
    fixture = TestBed.createComponent(AppointmentPlanningComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
