export function getCurrentDatetimeAsISO8601String(): string {

    const d = new Date()

    return `${d.getFullYear()}-${(d.getMonth() + 1).toString().padStart(2, "0")}-${d.getDate().toString().padStart(2, "0")}T${d.getHours().toString().padStart(2, "0")}:${d.getMinutes().toString().padStart(2, "0")}:${d.getSeconds().toString().padStart(2, "0")}.${d.getMilliseconds().toString().padStart(3, "0")}`
}

export function getTimeFromISO8601String(dt: string): string {

    return dt.split('T')[1].substring(0, 5)
}

export function getTimestampFromISO8601String(dt: string): number {

    let ts = new Date(`${dt}Z`)

    return ts.getTime()
}

export function offsetFromUTCISO8601String(str: string): string {

    let ts = getTimestampFromISO8601String(`${str}`) - (new Date()).getTimezoneOffset() * 60000

    return (new Date(ts)).toISOString()
}

export function offsetToUTCISO8601String(str: string): string {

    let ts = getTimestampFromISO8601String(`${str}`) + (new Date()).getTimezoneOffset() * 60000

    return (new Date(ts)).toISOString()
}