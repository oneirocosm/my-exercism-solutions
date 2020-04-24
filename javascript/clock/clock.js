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

    return (
      hours.toFixed().padStart(2, "0") +
      ":" +
      minutes.toFixed().padStart(2, "0")
    );
  }

  plus(plusMinutes) {
    const sumMinutes = this.minutes + plusMinutes;
    const newMinutes = mod(sumMinutes, MINUTES_PER_DAY);
    const noHours = 0;
    return new Clock(noHours, newMinutes);
  }

  minus(minusMinutes) {
    const diffMinutes = this.minutes - minusMinutes;
    const newMinutes = mod(diffMinutes, MINUTES_PER_DAY);
    const noHours = 0;
    return new Clock(noHours, newMinutes);
  }

  equals(otherClock) {
    return this.minutes == otherClock.minutes;
  }
}

function mod(n, m) {
  return ((n % m) + m) % m;
}
