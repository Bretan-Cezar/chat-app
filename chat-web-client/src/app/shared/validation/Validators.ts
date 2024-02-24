import {AbstractControl, ValidationErrors, ValidatorFn} from "@angular/forms";

const alphanumericNameRegex: RegExp = RegExp("^[a-zA-Z0-9]*$")

const forbiddenNameRegex: RegExp = RegExp("...")

const messageRegex: RegExp = RegExp("...")

export function alphanumericNameValidator(): ValidatorFn {
    return (control: AbstractControl): ValidationErrors | null => {

        const alphanumeric = !alphanumericNameRegex.test(control.value);
        return alphanumeric ? { alphanumericName: { value: control.value } } : null;
    };
}

export function forbiddenNameValidator(): ValidatorFn {

    return (control: AbstractControl): ValidationErrors | null => {

        const forbidden = !forbiddenNameRegex.test(control.value);
        return forbidden ? { forbiddenName: { value: control.value } } : null;
    };
}

export function forbiddenMessageValidator(): ValidatorFn {

    return (control: AbstractControl): ValidationErrors | null => {

        const forbidden = !messageRegex.test(control.value);
        return forbidden ? { forbiddenName: { value: control.value } } : null;
    };
}