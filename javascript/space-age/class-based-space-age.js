// space-age.js

export const age = (planet = "earth", seconds = 0) => {
  const ageConverter = new AgeConverter(seconds);
  const planetAge = ageConverter[planet]();
  return roundToHundreths(planetAge);
};

class AgeConverter {
  constructor(timeInSeconds) {
    this.seconds = timeInSeconds;
  }

  mercury() {
    const EARTH_YEARS_PER_MERCURY_YEAR = 0.2408467;
    return this.earth() / EARTH_YEARS_PER_MERCURY_YEAR;
  }

  venus() {
    const EARTH_YEARS_PER_VENUS_YEAR = 0.61519726;
    return this.earth() / EARTH_YEARS_PER_VENUS_YEAR;
  }

  earth() {
    const SECONDS_PER_EARTH_YEAR = 31557600.0;
    return this.seconds / SECONDS_PER_EARTH_YEAR;
  }

  mars() {
    const EARTH_YEARS_PER_MARS_YEAR = 1.8808158;
    return this.earth() / EARTH_YEARS_PER_MARS_YEAR;
  }

  jupiter() {
    const EARTH_YEARS_PER_JUPITER_YEAR = 11.862615;
    return this.earth() / EARTH_YEARS_PER_JUPITER_YEAR;
  }

  saturn() {
    const EARTH_YEARS_PER_SATURN_YEAR = 29.447498;
    return this.earth() / EARTH_YEARS_PER_SATURN_YEAR;
  }

  uranus() {
    const EARTH_YEARS_PER_URANUS_YEAR = 84.016846;
    return this.earth() / EARTH_YEARS_PER_URANUS_YEAR;
  }

  neptune() {
    const EARTH_YEARS_PER_NEPTUNE_YEAR = 164.79132;
    return this.earth() / EARTH_YEARS_PER_NEPTUNE_YEAR;
  }
}

const roundToHundreths = number => Math.round(number * 1e2) / 1e2;
