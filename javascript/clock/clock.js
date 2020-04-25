// clock.js

const HOURS_PER_DAY = 24;
const MINUTES_PER_HOUR = 60;
const MINUTES_PER_DAY = MINUTES_PER_HOUR * HOURS_PER_DAY;

export class Clock {
  constructor(hours = 0, minutes = 0) {
    const totalMinutes = minutes + hours * MINUTES_PER_HOUR;
    this.minutes = mod(totalMinutes, MINUTES_PER_DAY);
  }

  toString() {
    const totalHours = Math.floor(this.minutes / MINUTES_PER_HOUR);
    const hours = mod(totalHours, HOURS_PER_DAY);
    const minutes = mod(this.minutes, MINUTES_PER_HOUR);

    return Clock.formatDigits(hours) + ":" + Clock.formatDigits(minutes);
  }

  plus(plusMinutes) {
    const noHours = 0;
    return new Clock(noHours, this.minutes + plusMinutes);
  }

  minus(minusMinutes) {
    return this.plus(-minusMinutes);
  }

  equals(otherClock) {
    return this.minutes == otherClock.minutes;
  }

  static formatDigits(digit) {
    return digit.toFixed().padStart(2, "0");
  }
}

function mod(n, m) {
  return ((n % m) + m) % m;
}
