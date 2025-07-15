import { Component, OnInit, ViewChild } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { AppointmentModel } from 'src/app/models/appointment-model';
import { AppointmentService } from 'src/app/services/appointment.service';
import { PatientService } from 'src/app/services/patient.service';
import { CalendarEvent, CalendarParameters } from '../calendar/calendar.component';
import { PosssibleAppointmentModel } from 'src/app/models/possible-appointment-model';
import { AppointmentTypeService } from 'src/app/services/appointment-type.service';
import { AppointmentTypeModel } from 'src/app/models/appointment-type-model';
import { StepperSelectionEvent } from '@angular/cdk/stepper';
import { RoomModel } from 'src/app/models/room-model';
import { DoctorModel } from 'src/app/models/doctor-model';
import { MatStepper } from '@angular/material/stepper';

@Component({
    selector: 'app-appointment-planning',
    templateUrl: './appointment-planning.component.html',
    styleUrls: ['./appointment-planning.component.css'],

})
export class AppointmentPlanningComponent implements OnInit {

    readonly STEP_INDEX_DATE_RANGE_SELECTION = 1;
    readonly STEP_INDEX_APPOINTMENT_RESOURCES_SELECTED = 3;

    @ViewChild('stepper') stepper!: MatStepper;

    possibleAppointment: PosssibleAppointmentModel | null;

    range: FormGroup<{ start: FormControl<Date | null>, end: FormControl<Date | null> }>;
    appointmentService: AppointmentService;
    patientService: PatientService;
    appointmentTypeService: AppointmentTypeService;

    appointmentTypeFormGroup!: FormGroup;
    appointmentTypeFormControl!: FormControl<AppointmentTypeModel | null>;
    appointmentTypeModels!: AppointmentTypeModel[];

    startDateFormControl: FormControl<Date | null>;
    endDateFormControl: FormControl<Date | null>;

    calendarParameters: CalendarParameters<PosssibleAppointmentModel> | null;

    selectedDoctor: DoctorModel | null;
    selectedRoom: RoomModel | null;

    constructor(appointmentService: AppointmentService, patientService: PatientService,
        appointmentTypeService: AppointmentTypeService
    ) {

        this.startDateFormControl = new FormControl(new Date());
        this.endDateFormControl = new FormControl(new Date());

        this.range = new FormGroup({
            start: this.startDateFormControl,
            end: this.endDateFormControl,
        });


        this.appointmentService = appointmentService;
        this.patientService = patientService;
        this.possibleAppointment = null;
        this.appointmentTypeService = appointmentTypeService;
        this.calendarParameters = null;
        this.selectedDoctor = null;
        this.selectedRoom = null;
    }

    async ngOnInit(): Promise<void> {
        this.appointmentTypeModels = await this.appointmentTypeService.getAll() || [];
        let appointmentTypeModel = this.appointmentTypeModels[0];
        this.appointmentTypeFormControl = new FormControl<AppointmentTypeModel>(appointmentTypeModel, Validators.required);
        this.appointmentTypeFormGroup = new FormGroup({
            appointmentType: this.appointmentTypeFormControl
        });
    }

    get startDate(): Date | null {
        return this.startDateFormControl.value;
    }

    get endDate(): Date | null {
        return this.endDateFormControl.value;
    }

    async onStepChanged(event: StepperSelectionEvent) {
        if (event.previouslySelectedIndex === this.STEP_INDEX_DATE_RANGE_SELECTION) {
            this.onDateRangeSelected();
        } else if (event.previouslySelectedIndex === this.STEP_INDEX_APPOINTMENT_RESOURCES_SELECTED) {
            await this.onPossibleAppointmentRessourcesSelected();
        }
    }

    async onPossibleAppointmentRessourcesSelected() {
        let possibleAppointment = this.assertPossibleAppointmentSelected();
        let doctorId = this.assertDoctorSelected().doctor_id!;
        let roomId = this.assertRoomSelected().room_id!;

        let patient = await this.patientService.getCurrentUsersPatient();
        if (patient == null) {
            throw new Error("Current User has no patient assigned");
        }

        let appointmentModel = new AppointmentModel(
            possibleAppointment.appointment_type_id,
            doctorId,
            possibleAppointment.from,
            possibleAppointment.to,
            patient.patient_id!,
            roomId,
        );

        await this.appointmentService.create(appointmentModel);

    }

    assertPossibleAppointmentSelected() {
        if (this.possibleAppointment == null) {
            throw new Error("No PossibleAppointment was selected")
        }
        return this.possibleAppointment;
    }

    assertDoctorSelected(): DoctorModel {
        if (this.selectedDoctor == null) {
            throw new Error("No Doctor Selected");
        }
        return this.selectedDoctor;
    }

    assertRoomSelected(): RoomModel {
        if (this.selectedRoom == null) {
            throw new Error("No Room Selected");
        }
        return this.selectedRoom;
    }

    assertDatesSelected(): { startDate: Date, endDate: Date } {
        if (this.startDate == null || this.endDate == null) {
            throw new Error("StartDate or endDate was null");
        }

        return {
            startDate: this.startDate,
            endDate: this.endDate
        }
    }

    assertAppointmentTypeSelected(): AppointmentTypeModel {
        if (this.appointmentTypeFormControl.value == null) {
            throw new Error("AppointmentType was null")
        }
        return this.appointmentTypeFormControl.value!;
    }

    async onDateRangeSelected() {
        let { startDate, endDate } = this.assertDatesSelected();
        let appointmentTypeId = this.assertAppointmentTypeSelected().appointment_type_id!;

        let possibleAppointments = await this.appointmentService.getPossibleAppointments(appointmentTypeId, startDate, endDate);

        if (possibleAppointments == null) {
            return;
        }

        this.calendarParameters = {
            days: this.getDateRange(startDate, endDate),
            calendarEvents: this.parseToCalendarEvents(possibleAppointments),
            onDoubleClick: this.onPossibleAppointmentSelected.bind(this)
        }

    }

    onPossibleAppointmentSelected(possibleAppointmentModel: PosssibleAppointmentModel) {
        this.possibleAppointment = possibleAppointmentModel;
        this.stepper.next();
    }

    getDateRange(from: Date, to: Date): Date[] {
        let result = [];
        let currentDate = new Date(from.getTime());

        while (currentDate.getTime() <= to.getTime()) {
            result.push(currentDate);
            currentDate = new Date(currentDate.getTime());
            currentDate.setDate(currentDate.getDate() + 1);
        }
        return result;
    }

    parseToCalendarEvents(possibleAppointments: PosssibleAppointmentModel[]): CalendarEvent<PosssibleAppointmentModel>[] {
        return possibleAppointments.map(possibleAppointment => {
            return {
                from: new Date(possibleAppointment.from),
                to: new Date(possibleAppointment.to),
                data: possibleAppointment
            }
        });
    }

}
