import {
  AfterContentChecked,
  AfterViewChecked,
  Component,
  ElementRef,
  EventEmitter,
  Input,
  Output,
  ViewChild
} from '@angular/core';
import {CommonModule} from '@angular/common';
import {MatButtonModule} from "@angular/material/button";
import {ScrollSignal} from "../public-chat.component";
import {TextMessage} from "../../../shared/data-type/textMessage";
import {ReceivedTextBubbleComponent} from "../../common/text-bubble/received-text-bubble.component";
import {SentTextBubbleComponent} from "../../common/text-bubble/sent-text-bubble.component";
import {MatIconModule} from "@angular/material/icon";
import {MatBadgeModule} from "@angular/material/badge";

@Component({
  selector: 'app-public-chat-box',
  standalone: true,
  imports: [CommonModule, MatButtonModule, ReceivedTextBubbleComponent, SentTextBubbleComponent, MatIconModule, MatBadgeModule],
  templateUrl: './public-chat-box.component.html',
  styleUrl: './public-chat-box.component.scss'
})
export class PublicChatBoxComponent implements AfterContentChecked, AfterViewChecked {

  @ViewChild('scroll', {read: ElementRef})
  private _scroll!: ElementRef

  private _messageList: TextMessage[] = []

  get messageList(): TextMessage[] {
    return this._messageList
  }

  @Input()
  set messageList(list: TextMessage[]) {

    if (this._init) {
      this.setLastScrollLength()
      this.setLastScrollPosition()
    }

    this._messageList = list
  }

  @Output()
  public onInternalLockChange: EventEmitter<ScrollSignal> = new EventEmitter<ScrollSignal>()

  @Output()
  public onLoadMore: EventEmitter<number> = new EventEmitter<number>()

  public onLoadMorePressed() {

    this.onLoadMore.emit(this._messageList[0].timestamp!)
  }

  private _init: boolean = false

  private _lastScrollPosition: number = 0

  private _lastScrollLength: number = 0

  private _scrollLock: ScrollSignal = ScrollSignal.NONE_AT_BOTTOM

  private _newMessageCount: number = -1

  get newMessageCount(): number {
    return this._newMessageCount
  }

  get scrollLock(): ScrollSignal | null {
    return this._scrollLock
  }

  @Input()
  set scrollLock(scrollLock: ScrollSignal) {

    this._scrollLock = scrollLock

    console.log(`SL changed in chat-box: ${scrollLock}`)
  }

  public scrollToBottom() {

    let bottomPosition = this._scroll.nativeElement.scrollTopMax

    this._scroll.nativeElement.scrollTop = bottomPosition
    this._lastScrollPosition = bottomPosition
    this._lastScrollLength = bottomPosition

    this._scrollLock = ScrollSignal.TO_BOTTOM
  }

  private maintainPositionOnTopPush() {

    const newLength = this._scroll.nativeElement.scrollTopMax
    const newPosition = newLength - this._lastScrollLength + this._lastScrollPosition!

    this._scroll.nativeElement.scrollTop = newPosition
    this._lastScrollPosition = newPosition
    this._lastScrollLength = newLength
  }

  public checkBottom(): boolean {

    const currentPosition = this._scroll.nativeElement.scrollTop
    const bottomPosition = this._scroll.nativeElement.scrollTopMax

    return currentPosition === bottomPosition;
  }

  public setLastScrollPosition() {

    this._lastScrollPosition = this._scroll.nativeElement.scrollTop

    if (!this.checkBottom()) {

      if (this._scrollLock !== ScrollSignal.NONE) {
        this.onInternalLockChange.emit(ScrollSignal.NONE)
      }
    }
    else {
      if (this._scrollLock !== ScrollSignal.NONE_AT_BOTTOM) {
        this.onInternalLockChange.emit(ScrollSignal.NONE_AT_BOTTOM)
      }
    }
  }

  private setLastScrollLength() {

    this._lastScrollLength = this._scroll.nativeElement.scrollTopMax
  }

  ngAfterViewChecked(): void {

    if (!this._init) {

      this.setLastScrollLength()
      this.onInternalLockChange.emit(ScrollSignal.TO_BOTTOM)

      this._init = true

      return
    }

    switch (this._scrollLock) {

      case ScrollSignal.TO_BOTTOM: {

        this.scrollToBottom()

        this.onInternalLockChange.emit(ScrollSignal.NONE_AT_BOTTOM)

        break
      }
      case ScrollSignal.LOCK_ON_TOP_PUSH: {

        this.maintainPositionOnTopPush()

        this.onInternalLockChange.emit(ScrollSignal.NONE)

        break
      }
    }
  }

  ngAfterContentChecked() {

    if (this._init) {

      switch (this._scrollLock) {

        case ScrollSignal.NONE_AT_BOTTOM: {

          this._newMessageCount = 0
          break
        }

        case ScrollSignal.NONE_NEW_MESSAGE: {

          this._newMessageCount++

          this.onInternalLockChange.emit(ScrollSignal.NONE)

          this.setLastScrollLength()
          break
        }

        case ScrollSignal.NONE: {
          break
        }
      }
    }
  }

  protected readonly ScrollLock = ScrollSignal;
}
