export const colorCode = (searchColor: string): number => {
  let numVal: number = COLORS.findIndex(color => searchColor == color);

  if (numVal == -1) {
    throw new Error(`Color ${searchColor} is not a valid resistor color.`);
  }
  return numVal;
};

export const COLORS: string[] = [
  "black",
  "brown",
  "red",
  "orange",
  "yellow",
  "green",
  "blue",
  "violet",
  "grey",
  "white"
];
