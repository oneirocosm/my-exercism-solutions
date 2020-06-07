// space-age.js

const SECONDS_PER_EARTH_YEAR = 31557600.0;

const YEARS_ON_EARTH_PER_YEARS_ON = {
  mercury: 0.2408467,
  venus: 0.61519726,
  earth: 1,
  mars: 1.8808158,
  jupiter: 11.862615,
  saturn: 29.447498,
  uranus: 84.016846,
  neptune: 164.79132
};

export const age = (planet = "earth", seconds = 0) => {
  const earth_years = seconds / SECONDS_PER_EARTH_YEAR;
  const planet_years = earth_years / YEARS_ON_EARTH_PER_YEARS_ON[planet];
  return Number(planet_years.toFixed(2));
};
