export interface ITime {
    hours: number;
    minutes: number;
    seconds: number;
    milliseconds: number;
}

export const isTime = (obj: any): obj is ITime => {
    return obj && typeof obj.hours === 'number' && typeof obj.minutes === 'number' &&
        typeof obj.seconds === 'number' && typeof obj.milliseconds === 'number';
}

export default class Time {
    hours: number;
    minutes: number;
    seconds: number;
    milliseconds: number;

    constructor(hours: number, minutes: number, seconds: number, milliseconds: number) {
        this.hours = hours;
        this.minutes = minutes;
        this.seconds = seconds;
        this.milliseconds = milliseconds;
    }

    toString(_12_hour: boolean = false, format: { hours?: string, minutes?: string, seconds?: string, milliseconds?: string } = { hours: '2-digit', minutes: '2-digit' }) {
        const pad = (num: number, size: number = 2) => num.toString().padStart(size, '0');

        let h = this.hours;
        let am_pm = '';
        if (_12_hour) {
            am_pm = h >= 12 ? ' PM' : ' AM';
            h = h % 12 || 12;
        }

        let result = `${pad(h)}:${pad(this.minutes)}`;

        if (format.seconds) {
            result += `:${pad(this.seconds)}`;
        }

        if (format.milliseconds) {
            result += `.${pad(this.milliseconds, 3)}`;
        }

        return result + am_pm;
    }

    static fromString(timeString: string): Time {
        const [time, am_pm] = timeString.split(' ');
        const [hours, minutes, seconds] = time.split(':').map(Number);
        const milliseconds = 0;
        return new Time(hours, minutes, seconds, milliseconds);
    }

    getTime(): Date {
        return new Date(0, 0, 0, this.hours, this.minutes, this.seconds, this.milliseconds);
    }

    static fromDate(date: Date): Time {
        return new Time(date.getHours(), date.getMinutes(), date.getSeconds(), date.getMilliseconds());
    }
}