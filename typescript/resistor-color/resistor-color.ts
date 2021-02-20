export const colorCode = (searchColor: string): number => {
  let numVal: number = COLOR_MAP[searchColor];

  if (numVal === undefined) {
    throw new Error(`Color ${searchColor} is not a valid resistor color.`);
  }
  return numVal;
};

const COLOR_MAP: { [key: string]: number } = {
  black: 0,
  brown: 1,
  red: 2,
  orange: 3,
  yellow: 4,
  green: 5,
  blue: 6,
  violet: 7,
  grey: 8,
  white: 9
};

export const COLORS: string[] = Object.keys(COLOR_MAP);
