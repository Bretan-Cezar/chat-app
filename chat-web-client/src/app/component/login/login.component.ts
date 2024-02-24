import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {FormBuilder, FormControl, FormGroup, ReactiveFormsModule, Validators} from "@angular/forms";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {MatButtonModule} from "@angular/material/button";
import {PublicUserSocketService} from "../../service/public-user-socket.service";
import {Router} from "@angular/router";
import {
  alphanumericNameValidator,
  forbiddenMessageValidator,
  forbiddenNameValidator
} from "../../shared/validation/Validators";

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [CommonModule, MatFormFieldModule, ReactiveFormsModule, MatInputModule, MatButtonModule],
  templateUrl: './login.component.html',
  styleUrl: './login.component.scss'
})
export class LoginComponent {

  publicName: string = ""

  publicUserFormGroup = new FormGroup({
    name: new FormControl(this.publicName,
      [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(30),
        alphanumericNameValidator(),
        forbiddenNameValidator()
      ]
    )
  });

  privateUserFormGroup = new FormGroup({
    name: new FormControl("", [Validators.required]),
    password: new FormControl("", [Validators.required])
  })

  constructor(
      private formBuilder: FormBuilder,
      private publicUserSocketService: PublicUserSocketService,
      private router: Router,
  ) {}

  publicFormHasErrors() {

    return this.publicUserFormGroup.controls['name'].hasError('required') ||
    this.publicUserFormGroup.controls['name'].hasError('minlength') ||
    this.publicUserFormGroup.controls['name'].hasError('maxlength') ||
    this.publicUserFormGroup.controls['name'].hasError('alphanumericName') ||
    this.publicUserFormGroup.controls['name'].hasError('forbiddenName')
  }

  publicLogin() {

    if (!this.publicFormHasErrors()) {

      let name = this.publicUserFormGroup.value.name;

      this.publicUserSocketService.loginPublicUser(name!)
          .then(
              () => {

                let currentTime = new Date()
                let end_ts = currentTime.getTime()
                this.publicUserSocketService.requestMessageBatchUntil(end_ts)

                this.router.navigate(['../public-chat'])
              }
          )
    }
  }

  privateLogin() {

  }
}
