const COLORS = [
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

export const decodedValue = (colors = []) => {
  const colorDuo = colors.slice(0, 2);
  const colorDuoValues = mapColorsToValues(colorDuo);
  return colorDuoValues[0] * 10 + colorDuoValues[1];
};

const mapColorsToValues = (colors = []) => {
  return colors.map(color =>COLORS.indexOf(color));
};
