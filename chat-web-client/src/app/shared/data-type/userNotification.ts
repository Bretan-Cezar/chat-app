export enum NotificationType {
    Join, Close
}

export class UserNotification {

    constructor(public notificationType: NotificationType,
                public name: string,
    ) {}

}