import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {PublicUserSocketService} from "../../service/public-user-socket.service";
import {TextMessage} from "../../shared/data-type/textMessage";
import {ReceivedTextBubbleComponent} from "../common/text-bubble/received-text-bubble.component";
import {MatListModule} from "@angular/material/list";
import {SentTextBubbleComponent} from "../common/text-bubble/sent-text-bubble.component";
import {PublicChatNavBarComponent} from "../common/nav-bar/public-chat-nav-bar.component";
import {MatButtonModule} from "@angular/material/button";
import {FormBuilder, ReactiveFormsModule, Validators} from "@angular/forms";
import {MatIconModule} from "@angular/material/icon";
import {MatInputModule} from "@angular/material/input";
import {NotificationBubbleComponent} from "./notification-bubble/notification-bubble.component";
import {UserNotification} from "../../shared/data-type/userNotification";
import {PublicChatBoxComponent} from "./public-chat-box/public-chat-box.component";
import {getCurrentDatetimeAsISO8601String, getTimestampFromISO8601String} from "../../shared/utils/DateTimeConversion";
import {forbiddenMessageValidator} from "../../shared/validation/Validators";

/*
  Auto-scroll mechanism: Two ScrollSignal states exist - one in the chatbox, one in its parent component.
  These two states must be synced at all times.
  Whenever the chatbox needs to modify the state, it emits an event to the parent containing the new value.
  Then, the parent sets the new state value, which is inputted automatically to the state within the chatbox component.

  Usage of signal states:
  - TO_BOTTOM for auto-scrolling to the bottom
    * when a new message is sent
    * when the scroll to bottom button is pressed
    * when a new message is received while the scroll is already at the bottom

  - LOCK_ON_TOP_PUSH for maintaining the same view of the messages whenever older ones are loaded
  - NONE_AT_BOTTOM for indicating that the scroll is currently at the bottom
  - NONE_NEW_MESSAGE for signaling the chatbox whose scroll is not at the bottom that a new message was received
  - NONE - for indicating that the scroll isn't currently at the bottom

  The TO_BOTTOM, LOCK_ON_TOP_PUSH and NONE_NEW_MESSAGE signals are momentary, meaning once the scroll was performed,
  these will be immediately reverted to NONE_AT_BOTTOM or NONE.
 */
export enum ScrollSignal {
  TO_BOTTOM, LOCK_ON_TOP_PUSH, NONE_AT_BOTTOM, NONE_NEW_MESSAGE, NONE
}


@Component({
  selector: 'app-public-chat',
  standalone: true,
  imports: [CommonModule, ReceivedTextBubbleComponent, MatListModule, SentTextBubbleComponent, PublicChatNavBarComponent, MatButtonModule, MatIconModule, MatInputModule, ReactiveFormsModule, NotificationBubbleComponent, PublicChatBoxComponent],
  templateUrl: './public-chat.component.html',
  styleUrl: './public-chat.component.scss'
})
export class PublicChatComponent implements OnInit {

  public messageList: TextMessage[] = []

  public textInputFormGroup = this.formBuilder.group({
    text: ["",
      [
        Validators.required,
        Validators.minLength(3),
        Validators.maxLength(131072),
        forbiddenMessageValidator()
      ]
    ]
  })

  private _lastNotification: UserNotification | null = null

  get lastNotification(): UserNotification | null {

    return this._lastNotification
  }

  set lastNotification(notification) {

    this._lastNotification = notification
  }

  private _scrollLock: ScrollSignal = ScrollSignal.TO_BOTTOM

  get scrollLock() {
    return this._scrollLock
  }

  private set scrollLock(scrollLock) {

    console.log(`SL changed in chat: ${scrollLock}`)
    this._scrollLock = scrollLock
  }

  constructor(
      private formBuilder: FormBuilder,
      private socketService: PublicUserSocketService,
  ) { }

  ngOnInit(): void {

    this.socketService.newMessagesQueue.subscribe({

      next: (msg) => {

        if (this.scrollLock === ScrollSignal.NONE_AT_BOTTOM) {
          this.scrollLock = ScrollSignal.TO_BOTTOM
        }
        else if (this.scrollLock === ScrollSignal.NONE) {
          this.scrollLock = ScrollSignal.NONE_NEW_MESSAGE
        }
        this.messageList.push(msg)
      },

      error: (_) => {},
      complete: () => {}
    })

    this.socketService.loadingMessagesQueue.subscribe({

      next: (batch) => {

        this.scrollLock = ScrollSignal.LOCK_ON_TOP_PUSH
        this.messageList.splice(0, 0,  ...batch)
      },

      error: (_) => {},
      complete: () => {}
    })

    this.socketService.userNotificationQueue.subscribe({

      next: (n) => {
        this.lastNotification = n
      },

      error: (_) => {},
      complete: () => {}
    })
  }

  public loadMoreMessages(ts: number) {

    this.socketService.requestMessageBatchUntil(ts)
  }

  public updateLock(sl: ScrollSignal) {

    console.log(`SL update received in chat: ${sl}`)
    this._scrollLock = sl
  }

  public sendMessage() {

    const text = this.textInputFormGroup.value.text

    if (text) {

      this.socketService.sendMessage(text)

      this.scrollLock = ScrollSignal.TO_BOTTOM

      const currentDatetime = getCurrentDatetimeAsISO8601String()

      this.messageList.push(new TextMessage(localStorage['name'], currentDatetime, text, true, getTimestampFromISO8601String(currentDatetime)))
    }

    this.textInputFormGroup.setValue({text: ''})
  }
}
