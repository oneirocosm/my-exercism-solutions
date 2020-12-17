class Gigasecond {
  static milliPerGiga: number = 1e12;
  private timestamp: Date;

  constructor(timestamp: Date) {
    this.timestamp = timestamp;
  }

  date(): Date {
    const newSeconds = this.timestamp.getTime() + Gigasecond.milliPerGiga;
    return new Date(newSeconds);
  }
}

export default Gigasecond;
