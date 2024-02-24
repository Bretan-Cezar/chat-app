import {Component, Input} from '@angular/core';
import {CommonModule} from '@angular/common';
import {NotificationType, UserNotification} from "../../../shared/data-type/userNotification";
import {animate, state, style, transition, trigger} from "@angular/animations";

@Component({
  selector: 'app-notification-bubble',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './notification-bubble.component.html',
  styleUrl: './notification-bubble.component.scss',
  animations: [
      trigger('notification-transition', [

          state('visible', style({opacity: 0.25})),

          state('hidden', style({opacity: 0,})),

          transition('visible <=> hidden', animate('500ms'))
      ]),

      trigger('notification-display', [

            state('visible',  style({display: 'block'})),

            state('hidden', style({display: 'none'})),

            transition('visible => hidden', animate('500ms')),

            transition('hidden => visible', animate('0ms'))
      ])
  ]
})
export class NotificationBubbleComponent {

  private _userNotification: UserNotification | null = null;

  @Input()
  set userNotification(notification: UserNotification | null) {

    if (notification != null) {

      this._userNotification = notification
      this.setColor()
      this.setText()

      this.visible = true

      setTimeout(() => {
        this._userNotification = null
        this.visible = false
      }, 2000)
    }
  }

  private _color: string = '#000000'

  get color(): string {
    return this._color
  }

  public setColor() {

    if (this._userNotification != null) {

      switch (this._userNotification.notificationType) {

        case NotificationType.Join: {

          this._color = '#00FF00'
          break
        }
        case NotificationType.Close: {

          this._color = '#FF0000'
          break
        }
      }
    }
  }

  private _text: string = ''

  get text(): string {
    return this._text
  }

  public setText() {

    if (this._userNotification != null) {

      switch (this._userNotification.notificationType) {

        case NotificationType.Join: {

          this._text = `${this._userNotification.name} has joined.`
          break
        }

        case NotificationType.Close: {

          this._text = `${this._userNotification.name} has left.`
        }
      }
    }
  }

  public visible: boolean = false
}
