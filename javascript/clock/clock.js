// clock.js

const HOURS_PER_DAY = 24;
const MINUTES_PER_HOUR = 60;

export class Clock {
  constructor(hours = 0, minutes = 0) {
    const overflowHours = Math.floor(minutes / MINUTES_PER_HOUR);
    this.minutes = mod(minutes, MINUTES_PER_HOUR);
    this.hours = mod(hours + overflowHours, HOURS_PER_DAY);
  }

  toString() {
    return (
      this.hours.toFixed().padStart(2, "0") +
      ":" +
      this.minutes.toFixed().padStart(2, "0")
    );
  }

  plus(plusMinutes) {
    const sumMinutes = this.minutes + plusMinutes;
    const overflowHours = Math.floor(sumMinutes / MINUTES_PER_HOUR);
    const newMinutes = mod(sumMinutes, MINUTES_PER_HOUR);
    const newHours = mod(this.hours + overflowHours, HOURS_PER_DAY);
    return new Clock(newHours, newMinutes);
  }

  minus(minusMinutes) {
    const diffMinutes = this.minutes - minusMinutes;
    const overflowHours = Math.floor(diffMinutes / MINUTES_PER_HOUR);
    const newMinutes = mod(diffMinutes, MINUTES_PER_HOUR);
    const newHours = mod(this.hours + overflowHours, HOURS_PER_DAY);
    return new Clock(newHours, newMinutes);
  }

  equals(otherClock) {
    const equalHours = this.hours == otherClock.hours;
    const equalMinutes = this.minutes == otherClock.minutes;
    return equalHours && equalMinutes;
  }
}

function mod(n, m) {
  return ((n % m) + m) % m;
}
