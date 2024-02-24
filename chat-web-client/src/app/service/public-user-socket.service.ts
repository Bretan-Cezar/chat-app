import {Injectable} from '@angular/core';
import {Subject, Subscription} from "rxjs";
import {webSocket, WebSocketSubject} from "rxjs/webSocket"
import {TextMessage} from "../shared/data-type/textMessage";
import {getCurrentDatetimeAsISO8601String, getTimestampFromISO8601String} from "../shared/utils/DateTimeConversion";
import {NotificationType, UserNotification} from "../shared/data-type/userNotification";

const WS_URL = 'ws://127.0.0.1:8080/register-public'

@Injectable({
  providedIn: 'root'
})
export class PublicUserSocketService {

  private _socket: WebSocketSubject<any> | null = null

  private _newMessagesQueue: Subject<TextMessage> = new Subject<TextMessage>()

  private _socketSubscription: Subscription | null = null

  get newMessagesQueue(): Subject<TextMessage> {
    return this._newMessagesQueue
  }

  private _loadingMessagesQueue: Subject<TextMessage[]> = new Subject<TextMessage[]>()

  get loadingMessagesQueue(): Subject<TextMessage[]> {
    return this._loadingMessagesQueue
  }

  private _userNotificationQueue: Subject<UserNotification> = new Subject<UserNotification>()

  get userNotificationQueue(): Subject<UserNotification> {
    return this._userNotificationQueue
  }

  get isSocketOpen(): boolean {

    if (this._socketSubscription) {

      return !this._socketSubscription.closed
    }

    return false
  }

  private extractAndEnqueueTextMessage(json: any) {

    const textMsg = new TextMessage(json.name, json.datetime, json.text, false, getTimestampFromISO8601String(json.datetime))

    this._newMessagesQueue.next(textMsg)
  }

  private extractMessageBatch(json: any) {

    const batch: TextMessage[] = json.batch.reverse()

    for (let msg of batch) {

        msg.timestamp = getTimestampFromISO8601String(msg.datetime)
    }

    this._loadingMessagesQueue.next(batch)
  }

  private notifyJoin(json: any) {

    this._userNotificationQueue.next(new UserNotification(
        NotificationType.Join,
        json.name
    ))
  }

  private notifyClose(json: any) {

    this._userNotificationQueue.next(new UserNotification(
        NotificationType.Close,
        json.name
    ))
  }

  public async loginPublicUser(name: String) {

    this._socket = webSocket({
      url: `${WS_URL}?name=${name}`,
      serializer: (o: string | Uint8Array) => {

        if (o instanceof String) {
          return o as string
        }
        else {
          return o as Uint8Array
        }
      }
    });

    localStorage['name'] = name

    this._socketSubscription = this._socket.subscribe({

      next: (msg) => {

        switch (msg.messageType) {

          case "Text": {
            this.extractAndEnqueueTextMessage(msg)
            break
          }
          case "FixedBatch": {
            this.extractMessageBatch(msg)
            break
          }
          case "Join": {
            this.notifyJoin(msg)
            break
          }
          case "Close": {
            this.notifyClose(msg)
            break
          }
        }
      },

      error: (err) => {

      },

      complete: () => {

      }
    })
  }

  public logoutPublicUser() {

    this._socketSubscription?.unsubscribe()
    this._socket?.complete()
    localStorage['name'] = undefined
  }

  public requestMessageBatchUntil(end_ts: number) {

    let buf = new Uint8Array(9)

    buf[0] = 127

    for (let rank = 0; rank < 8; rank++) {

      buf[8-rank] = end_ts % 256
      end_ts /= 256
    }

    this._socket?.next(buf)
  }

  public sendMessage(text: string) {

    this._socket?.next(text)


  }
}
