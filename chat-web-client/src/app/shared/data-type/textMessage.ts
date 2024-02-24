export class TextMessage {
    constructor(public name: string,
                public datetime: string,
                public text: string,
                public sent: boolean,
                public timestamp: number | null | undefined) {}
}